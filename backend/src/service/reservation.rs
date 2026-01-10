use crate::error::AppError;
use crate::model::event_object;
use crate::model::reservation;
use crate::model::reservation_item;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::QuerySelect;
use sea_orm::TransactionTrait;
use uuid::Uuid;

#[allow(dead_code)]
pub struct ReservationBuilder {
    db: DatabaseConnection,
    event_id: Uuid,
    user_id: String,
    seat_ids: Vec<Uuid>,
    // Optional fields
    discount_code: Option<String>,
}

impl ReservationBuilder {
    pub fn new(db: &DatabaseConnection, event_id: Uuid, user_id: String) -> Self {
        Self {
            db: db.clone(),
            event_id,
            user_id,
            // Initialize defaults:
            seat_ids: Vec::new(),
            discount_code: None,
        }
    }

    pub fn seats(mut self, ids: Vec<Uuid>) -> Self {
        self.seat_ids = ids;
        self
    }

    #[allow(dead_code)]
    pub fn add_discount(mut self, code: impl Into<String>) -> Self {
        self.discount_code = Some(code.into());
        self
    }

    pub async fn reserve(self) -> Result<(), AppError> {
        let txn = self.db.begin().await?;
        // Maybe check for payment later
        let available_seats = event_object::Entity::find()
            .filter(event_object::Column::Id.is_in(self.seat_ids.clone()))
            .filter(event_object::Column::EventId.eq(self.event_id))
            .filter(event_object::Column::Status.eq("available"))
            .lock_exclusive()
            .all(&txn)
            .await?;

        if available_seats.len() != self.seat_ids.len() {
            return Err(AppError::Conflict(
                "One or more seats are no longer available".into(),
            ));
        }

        // let total_price: f64 = available_seats.iter().map(|s| s.price).sum();
        let total_price = 0.0;

        let reservation = reservation::ActiveModel {
            id: Set(Uuid::new_v4()),
            total_price: Set(total_price),
            user_id: Set(self.user_id),
            event_id: Set(self.event_id),
            status: Set("waiting".into()),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        let reservation_items: Vec<reservation_item::ActiveModel> = available_seats
            .iter()
            .map(|seat| reservation_item::ActiveModel {
                reservation_id: Set(reservation.id),
                event_object_id: Set(seat.id),
                price_at_booking: Set(0.0),
                ..Default::default()
            })
            .collect();

        if !reservation_items.is_empty() {
            reservation_item::Entity::insert_many(reservation_items)
                .exec(&txn)
                .await?;
        }

        event_object::Entity::update_many()
            .set(event_object::ActiveModel {
                status: Set("reserved".into()),
                ..Default::default()
            })
            .filter(event_object::Column::Id.is_in(self.seat_ids))
            .exec(&txn)
            .await?;

        txn.commit().await?;
        Ok(())
    }
}
