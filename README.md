# Filter_FASTQ_By_Average_Base_Quality

## Installation

    cd to-some-desired-base-directory
    git clone https://github.com/jelber2/Filter_FASTQ_By_Average_Base_Quality.git
    cd Filter_FASTQ_By_Average_Base_Quality
    cargo build --release
    
## Usage

Reads in from stdin, allows user to specify average base quality value to filter reads by, and outputs to stdout, other information is outputted to stderr:
For example:

```bash
echo -e "@1\n\ACTG\n+\n????" |./target/release/Filter_FASTQ_By_Average_Base_Quality 30

PHRED Quality =  Percentile
30            = 0th
30            = 1.25th
30            = 2.5th
30            = 5th
30            = 10th
30            = 20th
30            = 25th
30            = 50th
30            = 75th
30            = 100th
1 total reads
1 kept reads at average Q30

@1
\ACTG
+
????
```

For a real use case, one might do

```bash
zcat 0.pbccs-6.3.0.icecreamfinder-38.94.fastq.gz |~/git/Filter_FASTQ_By_Average_Base_Quality/target/release/Filter_FASTQ_By_Average_Base_Quality 30 > /dev/null

PHRED Quality =  Percentile
19.987514            = 0th
20.295076            = 1.25th
20.589355            = 2.5th
21.206745            = 5th
22.52666            = 10th
25.032253            = 20th
26.204447            = 25th
31.818943            = 50th
36.49015            = 75th
41.000195            = 100th
14411 total reads
8397 kept reads at average Q30
```

Then,
```bash
zcat 0.pbccs-6.3.0.icecreamfinder-38.94.fastq.gz |~/git/Filter_FASTQ_By_Average_Base_Quality/target/release/Filter_FASTQ_By_Average_Base_Quality 21.206745 |gzip > No-5th-percentile.fastq.gz
```

To filter out the lower 5th percentile.
