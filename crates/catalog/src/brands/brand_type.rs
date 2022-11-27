/// The different kinds for railway models brands
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BrandType {
    /// These manufactures produce models using the die casting method
    Industrial,

    /// These manufacturers produce models which are made of brass or similar alloys.
    ///
    /// They are usually more expensive than the industrial series due to the limited
    /// production quantities and the "hand made" nature of the production
    BrassModels,
}
