use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter, TransactionTrait,
};
use uuid::Uuid;

use crate::{
    error::AppError,
    model::{event_object, reservation, reservation_item},
};

pub struct UserSeatManagement {
    user_id: String,
    db: DatabaseConnection,
}

impl UserSeatManagement {
    pub fn new(db: &DatabaseConnection, user_id: String) -> Self {
        Self {
            user_id,
            db: db.clone(),
        }
    }

    pub async fn release(self, seat_id: Uuid) -> Result<(), AppError> {
        let db = self.db;

        let txn = db.begin().await?;
        // 1. Find the specific ITEM linking this seat to a reservation
        // We explicitly check 'user_id' via the joined reservation to ensure ownership
        let reservation_item_and_reservation = reservation_item::Entity::find()
            .find_also_related(reservation::Entity)
            .filter(reservation_item::Column::EventObjectId.eq(seat_id))
            .one(&txn)
            .await?;

        let (item, reservation) = match reservation_item_and_reservation {
            Some((item, Some(reservation))) => {
                if reservation.user_id != self.user_id {
                    return Err(AppError::Unauthorized); // Prevent hacking others' seats
                }
                (item, reservation)
            }
            _ => return Err(AppError::NotFound("Reservation item not found".into())),
        };

        // 2. DELETE the Item (The receipt line)
        // We convert to ActiveModel to delete
        let item_am: reservation_item::ActiveModel = item.into();
        item_am.delete(&txn).await?;

        // 3. FREE the Seat
        event_object::Entity::update_many()
            .set(event_object::ActiveModel {
                status: Set("available".into()),
                ..Default::default()
            })
            .filter(event_object::Column::Id.eq(seat_id))
            .exec(&txn)
            .await?;

        // 4. CHECK REMAINING Items (The "Last Seat" Check)
        let remaining_count = reservation_item::Entity::find()
            .filter(reservation_item::Column::ReservationId.eq(reservation.id))
            .count(&txn)
            .await?;

        if remaining_count == 0 {
            // CASE A: It was the last seat. Cancel the whole header.
            let mut res_active: reservation::ActiveModel = reservation.into();
            res_active.status = Set("cancelled".into());
            res_active.update(&txn).await?;
        } else {
            // CASE B: There are other seats left.
            // Optional: Recalculate price here if you track price in the header
            // update_reservation_price(txn, reservation.id).await?;
        }

        txn.commit().await?;
        Ok(())
    }
}
