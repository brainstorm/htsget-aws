# Serverless htsget

Proof of concept for a htsget serverless implementation, using AWS lambdas and Rust as a runtime.

Pre-alpha stage, some pointers:

https://github.com/samtools/htslib/issues/845
https://github.com/igvteam/igv.js/issues/349
https://github.com/googlegenomics/htsget
https://github.com/dnanexus-rnd/htsnexus

# Quickstart

To deploy the readId/variantId endpoints in the API Gateway, just run the **serverless.com** deployment script(s).

```bash
npx serverless deploy --profile default
```

# Backend

This proof of concept assumes that BAM files are serialized as `.parquet` files on a S3 bucket using ADAM's schema, some example files can be found here:

https://github.com/brainstorm/tiny-test-data/tree/master/wgs

To generate the `.adam` (which is just an apache `.parquet` file), I ran the following commands. One to transform the .BAM and the other to examine the resulting `.parquet` file:

```bash
$ adam-submit transformAlignments input.bam output.parquet
$ parquet-tools cat -j part-r-00000.gz.parquet | jq
```

## ADAM schema

The parquet/ADAM-serialized BAMs have the following schema as slurped by AWS Glue:

```
referencename (string)
start (bigint)
originalstart (bigint)
end (bigint)
mappingquality (int)
readname (string)
sequence (string)
quality (string)
cigar (string)
originalcigar (string)
basestrimmedfromstart (int)
basestrimmedfromend (int)
readpaired (boolean)
properpair (boolean)
readmapped (boolean)
matemapped (boolean)
failedvendorqualitychecks (boolean)
duplicateread (boolean)
readnegativestrand (boolean)
matenegativestrand (boolean)
primaryalignment (boolean)
secondaryalignment (boolean)
supplementaryalignment (boolean)
mismatchingpositions (string)
originalquality (string)
readgroupid (string)
readgroupsampleid (string)
matealignmentstart (bigint)
matereferencename (string)
insertsize (bigint)
readinfragment (int)
attributes (string)
```

So a typical SQL query for an incoming `htsget` id would be something like:

```SQL
SELECT referencename FROM htsget.adam WHERE referencename LIKE 'chr1';
```

# Development

Some handy dev shortcuts:

```bash
$ sls plugin install -n serverless-offline
$ sls deploy --profile default && sls invoke -f reads --path tests/rest/apigw_proxy_request.json
```

# Wishlist

1) If AWS/Google supported CRAM as they support Apache Parquet, the integration with other bioinfo tools toolchain, speedups and space savings could be quite remarkable.
2) Adding htsget support to IGV desktop should be relatively straightforward.