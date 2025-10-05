use bevy::prelude::*;

#[derive(Resource)]
pub struct Budget {
    pub income: f64,
    pub expenses: f64,
    pub balance: f64,
}

impl Budget {
    pub fn update_balance(&mut self) {
        self.balance += self.income - self.expenses;
    }
}

pub struct EconomyPlugin;

impl Plugin for EconomyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Budget {
            income: 0.0,
            expenses: 0.0,
            balance: 100_000.0, // Startbudget
        })
        .add_systems(Update, update_budget_system);
    }
}

fn update_budget_system(mut budget: ResMut<Budget>) {
    budget.update_balance();
    info!("Aktuelles Budget: {:.2}", budget.balance);
}
