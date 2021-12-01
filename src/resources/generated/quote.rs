// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::ids::{CustomerId, QuoteId};
use crate::params::{Expand, Expandable, List, Metadata, Object, Timestamp};
use crate::resources::{
    Account, CheckoutSessionItem, Currency, Customer, Discount, Invoice,
    QuotesResourceTotalDetails, Subscription, SubscriptionSchedule, TaxRate,
};

/// The resource representing a Stripe "Quote".
///
/// For more details see <https://stripe.com/docs/api/quotes/object>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Quote {
    /// Unique identifier for the object.
    pub id: QuoteId,

    /// Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,

    /// Total after discounts and taxes are applied.
    pub amount_total: i64,

    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    ///
    /// Only applicable if there are no line items with recurring prices on the quote.
    pub application_fee_amount: Box<Option<i64>>,

    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    /// Only applicable if there are line items with recurring prices on the quote.
    pub application_fee_percent: Box<Option<f64>>,

    pub automatic_tax: QuotesResourceAutomaticTax,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay invoices at the end of the subscription cycle or on finalization using the default payment method attached to the subscription or customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions.
    /// Defaults to `charge_automatically`.
    pub collection_method: QuoteCollectionMethod,

    pub computed: QuotesResourceComputed,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// The customer which this quote belongs to.
    ///
    /// A customer is required before finalizing the quote.
    /// Once specified, it cannot be changed.
    pub customer: Box<Option<Expandable<Customer>>>,

    /// The tax rates applied to this quote.
    pub default_tax_rates: Box<Option<Vec<Expandable<TaxRate>>>>,

    /// A description that will be displayed on the quote PDF.
    pub description: Box<Option<String>>,

    /// The discounts applied to this quote.
    pub discounts: Vec<Expandable<Discount>>,

    /// The date on which the quote will be canceled if in `open` or `draft` status.
    ///
    /// Measured in seconds since the Unix epoch.
    pub expires_at: Timestamp,

    /// A footer that will be displayed on the quote PDF.
    pub footer: Box<Option<String>>,

    /// Details of the quote that was cloned.
    ///
    /// See the [cloning documentation](https://stripe.com/docs/quotes/clone) for more details.
    pub from_quote: Box<Option<QuotesResourceFromQuote>>,

    /// A header that will be displayed on the quote PDF.
    pub header: Box<Option<String>>,

    /// The invoice that was created from this quote.
    pub invoice: Box<Option<Expandable<Invoice>>>,

    /// All invoices will be billed using the specified settings.
    pub invoice_settings: Box<Option<InvoiceSettingQuoteSetting>>,

    /// A list of items the customer is being quoted for.
    #[serde(default)]
    pub line_items: List<CheckoutSessionItem>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// A unique number that identifies this particular quote.
    ///
    /// This number is assigned once the quote is [finalized](https://stripe.com/docs/quotes/overview#finalize).
    pub number: Box<Option<String>>,

    /// The account on behalf of which to charge.
    ///
    /// See the [Connect documentation](https://support.stripe.com/questions/sending-invoices-on-behalf-of-connected-accounts) for details.
    pub on_behalf_of: Box<Option<Expandable<Account>>>,

    /// The status of the quote.
    pub status: QuoteStatus,

    pub status_transitions: QuotesResourceStatusTransitions,

    /// The subscription that was created or updated from this quote.
    pub subscription: Box<Option<Expandable<Subscription>>>,

    pub subscription_data: QuotesResourceSubscriptionData,

    /// The subscription schedule that was created or updated from this quote.
    pub subscription_schedule: Box<Option<Expandable<SubscriptionSchedule>>>,

    pub total_details: QuotesResourceTotalDetails,

    /// The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to for each of the invoices.
    pub transfer_data: Box<Option<QuotesResourceTransferData>>,
}

impl Quote {
    /// Returns a list of your quotes.
    pub fn list(client: &Client, params: ListQuotes<'_>) -> Response<List<Quote>> {
        client.get_query("/quotes", &params)
    }

    /// Retrieves the quote with the given ID.
    pub fn retrieve(client: &Client, id: &QuoteId, expand: &[&str]) -> Response<Quote> {
        client.get_query(&format!("/quotes/{}", id), &Expand { expand })
    }
}

impl Object for Quote {
    type Id = QuoteId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "quote"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvoiceSettingQuoteSetting {
    /// Number of days within which a customer must pay invoices generated by this quote.
    ///
    /// This value will be `null` for quotes where `collection_method=charge_automatically`.
    pub days_until_due: Box<Option<u32>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuotesResourceAutomaticTax {
    /// Automatically calculate taxes.
    pub enabled: bool,

    /// The status of the most recent automated tax calculation for this quote.
    pub status: Box<Option<QuotesResourceAutomaticTaxStatus>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuotesResourceComputed {
    /// The definitive totals and line items the customer will be charged on a recurring basis.
    ///
    /// Takes into account the line items with recurring prices and discounts with `duration=forever` coupons only.
    /// Defaults to `null` if no inputted line items with recurring prices.
    pub recurring: Box<Option<QuotesResourceRecurring>>,

    pub upfront: QuotesResourceUpfront,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuotesResourceFromQuote {
    /// Whether this quote is a revision of a different quote.
    pub is_revision: bool,

    /// The quote that was cloned.
    pub quote: Expandable<Quote>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuotesResourceRecurring {
    /// Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,

    /// Total after discounts and taxes are applied.
    pub amount_total: i64,

    /// The frequency at which a subscription is billed.
    ///
    /// One of `day`, `week`, `month` or `year`.
    pub interval: QuotesResourceRecurringInterval,

    /// The number of intervals (specified in the `interval` attribute) between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    pub interval_count: u64,

    pub total_details: QuotesResourceTotalDetails,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuotesResourceStatusTransitions {
    /// The time that the quote was accepted.
    ///
    /// Measured in seconds since Unix epoch.
    pub accepted_at: Box<Option<Timestamp>>,

    /// The time that the quote was canceled.
    ///
    /// Measured in seconds since Unix epoch.
    pub canceled_at: Box<Option<Timestamp>>,

    /// The time that the quote was finalized.
    ///
    /// Measured in seconds since Unix epoch.
    pub finalized_at: Box<Option<Timestamp>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuotesResourceSubscriptionData {
    /// When creating a new subscription, the date of which the subscription schedule will start after the quote is accepted.
    ///
    /// This date is ignored if it is in the past when the quote is accepted.
    /// Measured in seconds since the Unix epoch.
    pub effective_date: Box<Option<Timestamp>>,

    /// Integer representing the number of trial period days before the customer is charged for the first time.
    pub trial_period_days: Box<Option<u32>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuotesResourceTransferData {
    /// The amount in %s that will be transferred to the destination account when the invoice is paid.
    ///
    /// By default, the entire amount is transferred to the destination.
    pub amount: Box<Option<i64>>,

    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the destination account.
    /// By default, the entire amount will be transferred to the destination.
    pub amount_percent: Box<Option<f64>>,

    /// The account where funds from the payment will be transferred to upon payment success.
    pub destination: Expandable<Account>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuotesResourceUpfront {
    /// Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,

    /// Total after discounts and taxes are applied.
    pub amount_total: i64,

    /// The line items that will appear on the next invoice after this quote is accepted.
    ///
    /// This does not include pending invoice items that exist on the customer but may still be included in the next invoice.
    #[serde(default)]
    pub line_items: List<CheckoutSessionItem>,

    pub total_details: QuotesResourceTotalDetails,
}

/// The parameters for `Quote::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListQuotes<'a> {
    /// The ID of the customer whose quotes will be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<QuoteId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<QuoteId>,

    /// The status of the quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<QuoteStatus>,
}

impl<'a> ListQuotes<'a> {
    pub fn new() -> Self {
        ListQuotes {
            customer: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
            status: Default::default(),
        }
    }
}

/// An enum representing the possible values of an `Quote`'s `collection_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QuoteCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl QuoteCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            QuoteCollectionMethod::ChargeAutomatically => "charge_automatically",
            QuoteCollectionMethod::SendInvoice => "send_invoice",
        }
    }
}

impl AsRef<str> for QuoteCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for QuoteCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `Quote`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QuoteStatus {
    Accepted,
    Canceled,
    Draft,
    Open,
}

impl QuoteStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            QuoteStatus::Accepted => "accepted",
            QuoteStatus::Canceled => "canceled",
            QuoteStatus::Draft => "draft",
            QuoteStatus::Open => "open",
        }
    }
}

impl AsRef<str> for QuoteStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for QuoteStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `QuotesResourceAutomaticTax`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QuotesResourceAutomaticTaxStatus {
    Complete,
    Failed,
    RequiresLocationInputs,
}

impl QuotesResourceAutomaticTaxStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            QuotesResourceAutomaticTaxStatus::Complete => "complete",
            QuotesResourceAutomaticTaxStatus::Failed => "failed",
            QuotesResourceAutomaticTaxStatus::RequiresLocationInputs => "requires_location_inputs",
        }
    }
}

impl AsRef<str> for QuotesResourceAutomaticTaxStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for QuotesResourceAutomaticTaxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `QuotesResourceRecurring`'s `interval` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QuotesResourceRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

impl QuotesResourceRecurringInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            QuotesResourceRecurringInterval::Day => "day",
            QuotesResourceRecurringInterval::Month => "month",
            QuotesResourceRecurringInterval::Week => "week",
            QuotesResourceRecurringInterval::Year => "year",
        }
    }
}

impl AsRef<str> for QuotesResourceRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for QuotesResourceRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}