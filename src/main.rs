use std::env;

use stupidf::records::{RecordSummary, Records, records::Record};
use stupidf::test_information::FullTestInformation;

fn main() -> std::io::Result<()> {
    //let bytes = vec![3, 70, 65, 82, 5, 104, 101, 108, 108, 111];
    //let s = cn_from_bytes(&bytes, 0).expect("Failed to make Cn");
    //let s2 = cn_from_bytes(&bytes, s.len() + 1).expect("Failed to make Cn");
    //println!("{s}\n{s2}");

    let args: Vec<String> = env::args().collect();
    let fname = args[1].clone();

    let records = Records::new(&fname)?;

    let mut summary = RecordSummary::new();
    let mut test_info = FullTestInformation::new();
    //for record in records.take(5) {
    for record in records {
        summary.add(&record);
        if let Some(resolved) = record.resolve() {
            let header = &record.header;

            println!(
                "{}.{} (0x{:x} @ 0x{:x}): {:?}",
                header.rec_typ, header.rec_sub, header.rec_len, record.offset, record.rtype
            );
            if let Record::TSR(ref tsr) = resolved {
                //let test_information = TestInformation::new_from_tsr(tsr);
                test_info.add_from_tsr(&tsr);
                //println!("{test_information:#?}");
            }
            if let Record::PIR(_) = resolved {
                continue;
            }
            if let Record::PRR(_) = resolved {
                continue;
            }
            if let Record::PTR(ref ptr) = resolved {
                test_info.add_from_ptr(&ptr);
            }
            println!("{resolved:#?}");
        }
    }
    println!("{summary:#?}");
    println!("{test_info:#?}");
    Ok(())
}
