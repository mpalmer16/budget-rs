use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct BudgetCreationRequest {
    pub name: String,
    pub month: BudgetMonth,
    pub total_salary: f32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Budget {
    pub id: Uuid,
    pub name: String,
    pub month: BudgetMonth,
    pub total_salary: f32,
}

impl Budget {
    pub fn new(budget: BudgetCreationRequest) -> Self {
        Budget {
            id: Uuid::new_v4(),
            name: budget.name,
            month: budget.month,
            total_salary: budget.total_salary,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct BudgetItemCreationRequest {
    pub name: String,
    pub amount: f32,
    pub item_type: BudgetItemType,
    pub budget_id: Uuid,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct BudgetItem {
    pub id: Uuid,
    pub name: String,
    pub amount: f32,
    pub item_type: BudgetItemType,
    pub budget_id: Uuid,
}

impl BudgetItem {
    pub fn new(item: BudgetItemCreationRequest) -> Self {
        BudgetItem {
            id: Uuid::new_v4(),
            name: item.name,
            amount: item.amount,
            item_type: item.item_type,
            budget_id: item.budget_id,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub enum BudgetItemType {
    Mortgage,
    Bills,
    Food,
    Misc,
    Gas,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub enum BudgetMonth {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}
