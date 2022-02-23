use bio::io::fastq;
use bio::io::fastq::FastqRead;
use std::io;
use std::env;
use inc_stats::Percentiles;

fn main() {

 let args: Vec<String> = env::args().collect();
 let quality = &args[1].parse::<f32>().unwrap();

 let mut reader = fastq::Reader::new(io::stdin());
 let mut writer = fastq::Writer::new(io::stdout());
 let mut record = fastq::Record::new();

 let mut sum_records = 0;
 let mut sum_kept_records = 0;
 let mut vec = Vec::new();
 eprintln!("\n");

 while let Ok(()) = reader.read(&mut record) {
     if record.is_empty() {
         let check = record.check();
         break;
     }

     let mean_error_prob = mean_error_probability(record.qual());
     let mean_quality: f32 = -10f32 * mean_error_prob.log(10.0);
     vec.push(mean_quality);

     if mean_quality >= *quality {
         writer.write_record(&record);
         sum_kept_records +=1;
     }
     sum_records +=1;
 }
 let percs: inc_stats::Percentiles<f32> = vec.iter().collect();

 let arr = [0.0, 0.0125, 0.025, 0.05, 0.10, 0.20, 0.25, 0.50, 0.75, 1.00];

 for percentile in arr {
     let percentile2 = percentile*100.0;
     eprintln!("Q{} = {}th percentile", percs.percentile(percentile).unwrap().unwrap(), percentile2);
 }
 eprintln!("{} total reads\n{} kept reads at average Q{}", sum_records, sum_kept_records, quality);
}

 fn mean_error_probability(quality_bytes: &[u8]) -> f32 {
     let mut sum: f32 = 0.0;
     for q in quality_bytes.iter() {
         sum += 10f32.powf((q - 33u8) as f32 / -10f32)
     }
     sum / quality_bytes.len() as f32 // mean error probability
 }
