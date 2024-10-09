use log::{error, info};
use serde::Deserialize;
use serde_xml_rs::from_str;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

#[derive(Debug, Deserialize)]
struct GroupHeader {
    #[serde(rename = "MsgId")]
    msg_id: Option<String>,
    #[serde(rename = "CreDtTm")]
    creation_date_time: Option<String>,
    #[serde(rename = "NbOfTxs")]
    number_of_transactions: Option<String>,
    #[serde(rename = "InitgPty")]
    initiating_party: Option<InitiatingParty>,
}

#[derive(Debug, Deserialize)]
struct InitiatingParty {
    #[serde(rename = "Nm")]
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PaymentInfo {
    #[serde(rename = "PmtInfId")]
    payment_info_id: Option<String>,

    #[serde(rename = "CdtTrfTxInf")]
    credit_transfer_transaction_info: Vec<CreditTransferTransactionInfo>,
}

#[derive(Debug, Deserialize)]
struct CreditTransferTransactionInfo {
    //#[serde(rename = "PmtId")]
    //payment_id: PaymentId,
    //
    #[serde(rename = "Amt")]
    amount: Amount,

    #[serde(rename = "Cdtr")]
    creditor: Party,

    #[serde(rename = "CdtrAcct")]
    creditor_account: Account,
    //#[serde(rename = "CdtrAgt")]
    //creditor_agent: Agent,
    //#[serde(rename = "RmtInf")]
    //remittance_information: RemittanceInformation,
}

#[derive(Debug, Deserialize)]
struct PaymentId {
    #[serde(rename = "InstrId")]
    instruction_id: Option<String>,
    #[serde(rename = "EndToEndId")]
    end_to_end_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Amount {
    #[serde(rename = "InstdAmt")]
    instructed_amount: InstdAmt,
}

#[derive(Debug, Deserialize)]
struct InstdAmt {
    #[serde(rename = "$value")]
    amount: String,

    #[serde(rename = "Ccy")]
    currency: String,
}

#[derive(Debug, Deserialize)]
struct Party {
    #[serde(rename = "Nm")]
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Account {
    #[serde(rename = "Id")]
    id: AccountId,
}

#[derive(Debug, Deserialize)]
struct AccountId {
    #[serde(rename = "IBAN")]
    iban: String,
}

#[derive(Debug, Deserialize)]
struct Agent {
    #[serde(rename = "FinInstnId")]
    financial_institution_id: FinancialInstitutionId,
}

#[derive(Debug, Deserialize)]
struct FinancialInstitutionId {
    #[serde(rename = "BIC")]
    bic: String,
}

#[derive(Debug, Deserialize)]
struct RemittanceInformation {
    #[serde(rename = "Ustrd")]
    unstructured: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PaymentInformation {
    #[serde(rename = "PmtInfId")]
    payment_info_id: Option<String>,

    #[serde(rename = "CdtTrfTxInf")]
    credit_transfer_transaction_info: Vec<CreditTransferTransactionInfo>,
}

#[derive(Debug, Deserialize)]
struct Pain001 {
    #[serde(rename = "CstmrCdtTrfInitn")]
    customer_credit_transfer_initiation: CustomerCreditTransferInitiation,
}

#[derive(Debug, Deserialize)]
struct CustomerCreditTransferInitiation {
    #[serde(rename = "GrpHdr")]
    group_header: GroupHeader,

    #[serde(rename = "PmtInf")]
    payment_info: Vec<PaymentInfo>,
}

pub struct Pain001Parser {
    file_name: String,
    data: Pain001,
}

impl Pain001Parser {
    pub fn new(file_name: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(file_name)?;
        let reader = BufReader::new(file);
        let xml_content: String = reader.lines().collect::<Result<_, _>>()?;

        let data: Pain001 = from_str(&xml_content)?;

        Ok(Pain001Parser {
            file_name: file_name.to_string(),
            data,
        })
    }

    pub fn parse(&self, output_file: Option<&String>) -> Result<(), Box<dyn Error>> {
        let group_header = &self.data.customer_credit_transfer_initiation.group_header;

        println!("Message Identification: {:?}", group_header.msg_id);
        println!(
            "Creation Date and Time: {:?}",
            group_header.creation_date_time
        );
        println!(
            "Number of Transactions: {:?}",
            group_header.number_of_transactions
        );
        println!(
            "Initiating Party: {:?}",
            group_header
                .initiating_party
                .as_ref()
                .and_then(|p| p.name.clone())
        );

        let payments = &self.data.customer_credit_transfer_initiation.payment_info;
        println!("Number of Payments: {}", payments.len());
        println!("Payment Information: ");
        for payment in payments {
            println!("{:?}", payment);
        }

        if let Some(output_file) = &output_file {
            let mut wtr = csv::Writer::from_path(output_file)?;
            for payment in payments {
                for transaction in &payment.credit_transfer_transaction_info {
                    // add headers
                    wtr.write_record(&[
                        "Message Identification",
                        "Creation Date and Time",
                        "Number of Transactions",
                        "Initiating Party",
                        "Payment Information ID",
                        "Amount",
                        "Currency",
                        "Creditor Name",
                        "Creditor IBAN",
                    ])?;
                    wtr.serialize((
                        group_header.msg_id.clone(),
                        group_header.creation_date_time.clone(),
                        group_header.number_of_transactions.clone(),
                        group_header
                            .initiating_party
                            .as_ref()
                            .and_then(|p| p.name.clone()),
                        payment.payment_info_id.clone(),
                        //transaction.payment_id.instruction_id.clone(),
                        //transaction.payment_id.end_to_end_id.clone(),
                        transaction.amount.instructed_amount.amount.clone(),
                        transaction.amount.instructed_amount.currency.clone(),
                        transaction.creditor.name.clone(),
                        transaction.creditor_account.id.iban.clone(),
                        //transaction
                        //    .creditor_agent
                        //    .financial_institution_id
                        //    .bic
                        //    .clone(),
                        //transaction.remittance_information.unstructured.clone(),
                    ))?;
                }
            }
            wtr.flush()?;
            info!("Parsed data saved to {}", output_file);
        }
        Ok(())
    }
}
