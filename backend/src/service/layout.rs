use crate::model::event_object_position;
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};
// use sea_query::Expr;
use uuid::Uuid;

#[allow(dead_code)]
pub struct LayoutManagement {
    event_id: Uuid,
    db: DatabaseConnection,
}

impl LayoutManagement {
    pub fn new(db: &DatabaseConnection, event_id: Uuid) -> Self {
        Self {
            db: db.clone(),
            event_id,
        }
    }
    pub async fn move_object(
        self,
        id: Uuid,
        x: f64,
        y: f64,
        z: f64,
    ) -> eyre::Result<event_object_position::Model> {
        let db = &self.db;
        // Should save to redis first? and wait for period of time then commit
        let object = event_object_position::ActiveModel {
            id: Set(id),
            position_x: Set(x),
            position_y: Set(y),
            position_z: Set(z),
            ..Default::default()
        };

        let model = event_object_position::Entity::update(object)
            .exec(db)
            .await?;

        Ok(model)
    }

    // #[allow(dead_code)]
    // pub async fn move_objects(
    //     self,
    //     seat_ids: Vec<Uuid>,
    //     delta_x: f64,
    //     delta_y: f64,
    //     delta_z: f64,
    // ) -> Result<Vec<event_object_position::Model>, AppError> {
    //     let result = event_object_position::Entity::update_many()
    //         .col_expr(
    //             event_object_position::Column::PositionX,
    //             Expr::col(event_object_position::Column::PositionX).add(delta_x),
    //         )
    //         .col_expr(
    //             event_object_position::Column::PositionY,
    //             Expr::col(event_object_position::Column::PositionY).add(delta_y),
    //         )
    //         .col_expr(
    //             event_object_position::Column::PositionZ,
    //             Expr::col(event_object_position::Column::PositionZ).add(delta_z),
    //         )
    //         .filter(event_object_position::Column::Id.is_in(seat_ids))
    //         .exec_with_returning(&self.db)
    //         .await?;
    //
    //     Ok(result)
    // }
}
