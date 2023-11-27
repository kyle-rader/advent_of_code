use std::collections::HashSet;

#[allow(dead_code)]
fn part1(input: &str) -> u64 {
    let mut sum: u64 = 0;
    for line in input.lines() {
        sum += prrority_of_bad_item(line);
    }

    sum
}

fn prrority_of_bad_item(line: &str) -> u64 {
    let mid = line.len() / 2;
    let first: HashSet<char> = line.chars().take(mid).collect();
    let second: HashSet<char> = line.chars().rev().take(mid).collect();

    let suspect = first.intersection(&second).next();
    if let Some(suspect) = suspect {
        priority(*suspect)
    } else {
        panic!("No duplicate item found!")
    }
}

fn priority(suspect: char) -> u64 {
    (match suspect as u8 {
        n if (97..=122).contains(&n) => n - 96,
        n if (65..=90).contains(&n) => n - 38,
        _ => panic!("Bad input!"),
    }) as u64
}

#[allow(dead_code)]
fn part2(input: &str) -> u64 {
    let mut sum: u64 = 0;
    let lines: Vec<&str> = input.lines().collect();
    for i in (0..lines.len()).step_by(3) {
        let a: HashSet<char> = lines[i].chars().collect();
        let b: HashSet<char> = lines[i + 1].chars().collect();
        let c: HashSet<char> = lines[i + 2].chars().collect();

        let inter1: HashSet<&char> = a.intersection(&b).collect();
        let inter2: HashSet<&char> = a.intersection(&c).collect();
        sum += priority(**inter1.intersection(&inter2).next().unwrap());
    }
    sum
}

#[cfg(test)]
mod tests_y2022 {
    use super::*;

    #[test]
    fn priority_works() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('p'), 16);
        assert_eq!(priority('A'), 27);
    }

    #[test]
    fn parse_line() {
        let line = "vJrwpWtwJgWrhcsFMMfFFhFp";
        assert_eq!(prrority_of_bad_item(line), 16);
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), 7597);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), 2607);
    }
}

#[allow(dead_code)]
const INPUT: &str = "zBBtHnnHtwwHplmlRlzPLCpp
vvhJccJFGFcNsdNNJbhJsJQplQMRLQMlfdfTPCLfQQCT
GPhjcjhZDjWtnSVH
BNhHVhrGNVTbDHdDJdJRPJdSQQSJwPjR
lvtsfbsqzwSnJcvjSm
MftttFLftZMLgtgMbltMqZzbDNrTpVGhNWrDTrpTGNpZGZhD
VSSHcTgTtTdtllZlzmmbljTn
RqMqsFfQLLFLQFMMfRLPZLvPpCfWrbpmCbjCnfjlWmnrmmnm
hqRDqPDRsqNHwtHSNBZtJd
tNFDpDFrtdjfmjjjFmFFdScpZhZScTJgpHccHhMJgS
lLzSlSCQqbsVhBghggBZgCcJ
zRLVVLQnvQqVVzRldfWrwffjjdwSdfjv
bpWqqqWvHBpwGBCCRl
hJdjdJFQqdBBDMMC
tFFzJZFtJSqtZJQsWLbNSTnffHfvTH
lFhRZhFjPlqMlJqZJlJcRLwrLrwStRwtsVVtVSrgRV
WcpDvDfBmpDHzWBDbpbmWmNVSSTzLTtrVswgttVVzwwr
pbWfmGBpHfDmWnvvGbmWnjjMqPJMlMFPdGcjqPqPhP
NjFNRlpVLFCSSlbBWWfw
pssPZQQsMnzmtnQPttzDBbBJBcrrJWbrZSBJSbfC
QTHPHspMNGHdhvRR
QfPdSJfFJmthSthtwbsNLbPLlLTLpbvP
nHnMBnZqqgBMnWrZMqnZVcbCqRwNsvblRwppbllTsRNp
nZHBHznMnWgcrnVBtjFdfmzQNtNddjNF
hFhfPghppPhpRNhzsjsvHVzjpsGnWz
tTjlCCwMqtdMjMctGJWHwWnVwWnwvWGs
rZdrjBBtqdCtlcdgFZQLfhRLFSgRNP
RDHSWrJWffJFlJCgCMCDjCvzjPMP
QtGTndBwBtNzBVjBCMgB
LdwwMpTdwsRHsqSHqHJl
RfsfzvLLFvFzCSvSbDsTpTGMPMZPPTMt
jqWBjwBBNwWqwPGZbTwVwVtD
BnhgglhhNNngqjBjHNWrZLlFLSCJSFFCCQzQvQFCFF
HLvLDQbvnDQDvbHTLhntSnGBSlfGldddcmfMMf
NgFjZjrZZJrlfJfSVcBJGc
scWCNFZpsjzrDLwLhbQzhQwD
SlqJlThDPqpwSTwhcbDdbWDbZGcZNcDb
MsnWWjHjvLvfscjjgdzNdbgbcc
vQQvWVQFLLHfHVBWfsfmFFpJRhhSplqlRJqpBwlqTCPC
DZbDzzZDjQbPGZFFSSgSlFCzTgzm
qLnvwvhddrqMrwrCTLLFJjmtSlFlSH
VdhvsWqdVWvvRhsvqbpbPcZfPpjZGBQNRj
mJNtNFmzDZtzdzrLtwwRqJSchgfGcRfwRB
pWpjQjCTQnHMWCCpjQpHvTqcwTwScfRcBcSGBRThwS
MQHjvjVCCqsvljWnVQzLtNPZzmzLVNLddtPN
QVRPRVDgsRjLssnL
TTGDJDJfbfLHSnsMWWbs
qGqqTFFDqgQgQQQq
nlMnRRjbMjCdJVQJCZ
nGqfLwfNLFNLnPPGFVVCdVGZJtCtCCVzJz
LHHfPNHnPqqLwqPqDPWfNFvMglbhhbMgmclgcllDmgmrcl
cLLWWSThtdLpRcddcgPRZFDMCVPPMCCPCPCZ
NfGbGNzrBNffGNJjbPPZsZmZZPmDHpMH
zlJBfzlQzNjNjfJcpwSdvWhcvLwQWt
cVVQfVCJVrVcTJnfNvlDFmDrmlvrFWlL
snZHpMhZtMbtPNvzHWWvNFNvNW
gppnbbbRgMnZbswRqRwbqTcCCSTCJJdGjgfVGTdcCG
jplgNdrHrrNZgdHmlHNJHddlDSPPSTlzTSlTSDSzCQLfzf
vscvWWWvGWGGscbFMpRWFwQTPzfLQwQwPfLbzSzzDL
GvGBWpqcMVRNNZHgdHdtBJ
LchbZhjjZFjwSmPRqRffqbdtggdR
vWHMWlHJdGqtRqHV
MvzCJlnMnlTNnNNLLdhjjCdjjhDjjL
FNCllHFvCGvwQcPQJfgfmwgh
zjtRpbDLjtsrzbLLQmfBTgTBQQfhbfQB
WLgqRzqsrWvFGFZFZC
qjLlNcLjcNWpQLlQMmvmhCvCgsMZZghj
tGSDJtRGJzHMMGDVZCfvmfhzmZZgZsmv
BSSRDRHBGHtSSSbGJSwHbNcLQddqMNlrqcMQMldBWc
JSfctrtctDpszHvzVQHr
glCWjhWmFjlmlhmdWPhVVznvcHjszbvvpHvznv
FgBmFhCBCGFqglgmhCFmSTSRLJLLZfSRJcDSGMtM
vZGlFFtLMLdShSSShRVtVf
rQNvmznWPNCPNsrCsbWbsPCvjShhhfHBBHJjSJRhjSRnHhSj
mCNsQCmqszNcQzrzrrzWvGgGMgpdFpMLlFZGwcLDdg
QJRJQDlcqLlWbNGL
HCnwwsCrnstLWqtWNgZNgg
rsnTrTCHTnnVwnsVPqqDQcRjcczMPvPRzM
qCzjqnzVdzrdhnhddDbDBMPttcGBDBDPnc
sZgRQWHgWHHLsgsRRZsJbpJlDcDGNcTDFtGNFFcJNFPBPBTc
WggbRQSRRgRSsWWmbHqvVffVwhzvCdmfhmdV
lhqWcNpQGcNmmHmNPWCsQzQsgrQrBMCMbMVM
wDLFFDJvSFFZRDZSzCrzTzsRgVWbCrMW
dFwDtZfdjFZWFFfmHGPnPPmqfmPNcN
lcMRNJRGGLJnNVFbVrwrwZrD
tjCzQjQhQwgWFShVFS
ffHQsQssQTzBsPnLpMPRwsJP
MQSMSBSRFMQLJChLChjTBh
WmVlPrwnpwDlflNpDrNnDlDwThJCCdLJhhdhCfJTccGjvscd
gnDVnNnwgglwDwptSZFzgQHqbjZgZZ
nwBcFgwTDcNrpZMD
WQWCLZmvhMRvNjsNSD
CGGWmZGHHhtVzHbTqgTdbgzz
RmcTCwvssRbsThTcVRJJfSPqfJwJFqfjfMFq
zQNZDWtQlDZGBQPfFQqjJLjL
rrglggZGWnrnrrHlDhsbsPTVCsCVsTRpHv
wFGfzSvCPGttSzqwmtqmvvPRDDRCWgWWDTBTMcBcBWbCRM
hVJJHQHnpWnDTNnnDb
LJsVVdhQqvmdbbSf
srlJztzsVVsSsVtRlNllTWzzmqGhqWLPCDCgmChPLDdqCmCP
bZQMZpbvMBMgmDGmZLSPZd
MpScMSMpvfjMBcBcfMfSBnzlTjssNszrNrtlTVzlzFVN
rCtgrgClprGGClnJCZmwtMjZRjbjjcjZQv
PWVfBHWPdbNfbbRmRj
sPsVqFPsHWLhBVVqHFqPVddWSDLJgpTCnnrRRLGpJSSTRrgT
zjqpGjrQjGqSHCVvCrRZDN
cTdshMhdmcMNmddRHBhvCCBCCvHZDC
JTmTmJnLTdwzNQpPWJWgpP
BmpZmrzZnznHbpprSbQSQbqdSVqbPQcV
fRGTGJZRTTDwJTJRGDfgJgNFlSSFcldfdccFVlPlFFQPSQ
GvTTTZZLmsntzmCL
VhMcrmbhvzMSnhvftbRbllLtglBBtf
HqqqJqDqPjJPNjjDVFDZCdqBtRtGBGlGRfQQgttQfHlTQl
pCZJPqqZpmhvhpVh
dWLBJHJhGJGMBJRcDLDSQsSQpvcR
ZlnnPqglblfRRpSvSsnz
sPTgZVjjmwVTljrwTTlbwVGdJhBNNdFdMGNHHJMjBNFN
FhFrfbfgbLRdfqfrmvDgLdjrcQtSNStHHHQlSjJJPllt
CnspzZWTpCnMVzzZZGZRCzttHNjNlQlSNtNlNjVcjlQS
GCZsZBRwnvwfbqwFwb
bZnJFJgLFRnqQZqJQJFQGpCLNcGlLllClNtccjGc
rVfvwPDhPHGtlcbClr
mBhshsfMvBvqsQJdTbgnqQ
jgWHqMSWMGqWjWjqbWGJQDfVqLfrfDfJhVLfTr
pPplwsRZPFZFtLhfwgfwrhJL
zlRsdgFcRgmjdBCMHdjHWB
qJSGJSPQWzcprtQZtt
mBMVfsNBnZzcNtcc
LMLBsmMlvBgFsghVVvfgLBvbJJSqgGHqPGPtCWwbJHqCPG
ZvZLcdMGVMlHDvDpvqhH
NNSrQNbJbrTnnWZDDZqqhqpW
wbgNJrsrCwwJQZbsrJBFzjCCdzGdjcGzMdzj
JbVmdVLJJJdQMnzmmMgHjPqqjNgvqwngHNNP
ZfffDZZsRpcpRDcCRrlpplcWSSgwgSwjvvsjPSwhNSWggh
cCtfppZrpjtMMmdQQTLz
TtbnmbdmTmgTlPNhqvqj
wrwrLsVZRsJJJsfHjvPPWfhjHqRN
sDZwDvsCCQLJZQJQsMCMzZBtSMpndcSFnnSBFtSBmdBc
mWFTZdmQdZFrFQbCRsrspjSjnvCLRS
GwlDqcNHDzwGfHSRqCgJsSpnvpSL
NGlcNwHLLGfDDHDhDwDcwVczbPddZtMFWttWWtdPPdQdhPWd
mnfcZgcdZqnqdfFqPmHfhqsbgVMCJNMtvCJtMvtblTJtvb
rRLDDjPSjjPDGBQSBNbtLVtbMNNJlTMtbl
SzjDDzRRpGQDDDPHzdsmnnhsqcqdFq
ZDGNRDGjSdwnnmnsVNsHJJ
tMBWWrddLPLhvWTTPLccvmmbVpgsJHmccppJ
ClPrtBWWrhrFLBPlCRzjzGqdRzjRdRGZjF
csTRNQNJcNBDLfhfMf
qGmWpGHqrqPLChPRhVFPDD
tgHrtnrrJnZRTZcv
FLqrfmLDrqCmqjTqcbGqRTGVvb
FMtWMSWzzFStJzPzhWzhQvTvHVjjTjHTTHvbHc
PgtWWstWtSpZWPzWwnrBsdBDdFLfllLlfC
mThbMDMQDCDbwLqWpqPpdhwR
zgrcffgHNZltZSgHLsRsLLWRWgLqppsW
SVlSrfSHlSSVlrJfVctlNDMCmMFbnbRDbDBFJFbBRM
PrBrWqtRPdBLLrBwqpswgpwhgpnZhhzsgw
FTFRSVJQVJflFfQQgggGMZngGQZszZ
TbmfFJFSDFblSTDSFFbmVSDrPLLWtcmBqqRmBtmcLtcrjP
DjPsMwDjLVVTsvNNRTNTRT
ztdQQHqHlFNtfRNNNMgg
FzhMhHQlDcCrhCCc
zSHGzzmHgnnMDLTNTG
lPVBtvhQjpNSMWTLBD
VCftbjvbVCfPbZwsJsrSgSSZwC
CbwgmvMnmnCwMmwRQqJBGBgHZHpJHdtdZpJt
zVSlNSDlrzNhqlNTScDzVWfBBZZZZGBstGsdsWFpdHdJsW
NDlLzhrVcqRPCMRwLLLw
TjTHHLwnLjVlTwLjgVfvsFvDsdWfvDvFMd
qbRRRpmpcmDcczppztSqSvWFssFGfWdMvfQWdfsG
RZpqDBmtrzhzphjTgjHlnwjgJhgJ
dLmMgdgzwDLzDWFhBWvzFzzBZJ
tTVcppbSTfstTMMHfTbhBchhJFCWcjWBZhjGGB
SSSSNbsNRpRRsRrfVHfRpNtlPgQDLPdMmlDLlrPnqPdPLl
qqbTCSqdqqFZdRLZhwhZ
HWWlHtlrBfGtVssnsLnHfJVPPMMFzhPRwMPwFhzPZzPMGM
nfmtsrlsnrfVnHJrVBWlsVfgbbNTNSvmvvpcTjLjLbqvvS
GGhFvGPFcThqffPdnfNLqZZCSwtQSwZpwQQBsL
RglMRrJJgHBCBZSQQpdr
WmbRHHbzDgJMDzRDMdWmWHzHNFFvvGGhnvVvvfcvnFfcbvnT
QsfQmsLfZZZcshnJ
dSgdWgSVVFvzSpqFdqTgWRHbJNcbZNCTJCNNZRRCCh
FcpVjgDvVVFdVWFvzjwwQtBMLtBBGDwftPrB
rqsRrHsvsPqswNcJcNJrnnBrNn
bFjgGFdbVRNNnpRQpV
GSthhggGDSvMRqtHvMfM
ZwVPgMsgVsGzVsRZpgpzzgpFMrNbbLFrDLFFrrSDLfrNBN
qvnjBhQhntbfDLrF
CJlHHcHcTWqvpBdsWRpdPdgs
BjmTDjJBCBWrgQRPFlWWlW
dHphshtdtVHVhpJqspdvRrqFPgrLPPFPrrRPvQ
sdMsMtStVszpwMzHjJGjCcZjmScNfCDf
DmGdDffgDSDDdJstqdJldlRt
MhnvMCZCbbZHMvsCHtrcVrPjJcRqVtlt
LsQbsFZvZhQzZwhQWTNgBWpNwSGpTmfS
RRJQnCzbZZLTZJCBtWvFtsfqBqtfWb
prjlChGNldGNdlSVMhWfqWtfsvwvqsFtdtsq
GGjNDNhpMGMGVhrnZZTzcTHCCJcDHc
RmbMmjgpPjMBsBMfchhVsc
HwFWFTztSrtFpcQvBsSqVscBBC
zWwnJFHtWWHDgbGgdpGpnl
mnbWbRRLRFnmmWcCDTBVwCDBlwNW
ggJPtpdHGfdZtMHgtZgVPPBCVsPNBcsBTTDDCC
hpvJJTpGhdhtJdMHqvmmnLvSbmnFnRFm
WWtrWrNgVbRjMrQCNzqJFwQJFNTJ
LdHPhcdchQQssLzJrz
pBccnHpnrrcGHnnSlWjnRMSlbt
NMMfNFnZgMVThhTMcgTDJDJjsVvvJJqJmHsqHG
LQpwwprCQzBNBdGjGjHswswdvm
CBCzzCrbWbSlNQnTRgPPfFRWnfgc
RFwHVQRwFgTQSFVhdsdHsBdDBnnqnq
LGftLtPGGMzlNrhlPqPsrJ
fvGpWpMtccpTwwpRRQhh
TTJCGdTGtZRQQCnzcnCv
FWWHPSFNFbDbDDqSWnVmLRRjRRQLhcmLjS
qPwPWwFppbwggGZGfdJZgdnGdd
zSTWzrzWTLWpCtCGpqqGgplc
nZWwsJVZZBnJHJCclHllgtChgCgc
DFnVBJsFssVVFBFnBdfvjDSmTMWzrmMfRmTv
MJmgMssrsggqqMVstbwTcTbPbTTwThmw
NRBBGRjHVRRcRbCp
QnSfzLWzNHzNVQQVjrglJMsMFvgJdFWrgZ
ggLLGnhgnPvJHZnN
VBtmVSldbSBVlcNPHvjmNcwNZZ
tdWqSVSSBztVWGrThLhfrfvG
TDqrjdSwLqDppdTCdzPBFmmjQmhHFPFQhPFR
zlGbMcVcVtsPHFRhWRRsPF
btgvlVVcDZZZqgrz
DgwlgbbFDDjjPTHDrmddPhPV
WqtMBBtQsttMNWQBqsbJpGGzdPdTHLVmTzJhmTPhHHPTmH
qQsqGZNQtZGMNsNtZpFnjnCRbZffwwSRljFf
gMdFLCdnMZCTFFCqnTgWLCHfSgPgPHStcQQmfSBBSfHg
vrwwrwzbGjjswjvhGGsjPQmqRmHPbBtcBQtqfmcH
qzJllVsGVGljjsrzwDzhwzDGTddNLFnZWNdpCVWTNTZTLZCF
LtwMhDtctwbwwppdWBJQJBWPvPfDfqvG
FTzrNrgSRFrgzFRHNVFQJvlqHjBvQWlQWqPBfq
sFgNzmVmNzgTvVTMwhMhstMwZtsbsc
MrBDQVzzlrvhQzQrDMVQrzrzgRJnRRwwRbwSwwVRRNSgwwwJ
qFTPTvfTHcqqncpcwR
LmtdGGPmTPGCTLHLWsZMhvZMMMzrzzdlMQ
ZVNpjfpZNpfNgNjzNVfWtnbbWmBHtsZWBSZBGS
MrDrQvvDrPLDMvFvdmBGGsBBCtsHrnrGCm
ltRMwLLDDRlvQwvlQcwhqfcJNpgzjJpjhJ
sRRRlRbcFbBBdnFBwCGppNvGrTCDDGVNlr
PPSLQzHjzZZPLZPjgTNTgpCbVJvGrNCTGr
ZLHHPQjhQmWWSRRnssdtbnmfwF
GRwrMrHJGwJPGWsgfqQgsc
VbTvLQCZLSWWsgWf
TVDvVCvppvTDmzZVTbZpTzBBNQQQJlJBBJBNNJmRBwRH
shJRWJsjZGNjSTrjFS
dMLCddggldQzMCCVgzVVLmLvTwNFFSqpNSqSbFGSqTTpMTFN
VGQvVglCLcVzgdddCDVvlsPZRRBDJPHZWZZnBsWJRR
CrwlwhRCMrswnsHBFccHHWFc
QJTmtfQgLtzQfLQfdPcWSFHHDDSpcFpFBg
jTQTqbfQfmLbLQJbJrRCWjljZGjNrZlZlC
JmthDmLShtJmHphphJQCwjdjdFDzFgzFdgdNlC
sbMTVBrWMbNvVMnsWMnVzjsjwCfjFgfZzfdgdzlj
NvqbbBcMMPPSqLSpGGthmp
RfGWFHlPFFNWGFZRZBjvwCvzBwhhrvvjzmrr
sLJSLMSTSJTbStJtMSqSqbpMrvmrzWdvhmjDCzzwrrpjdDDv
SbQqsqsWcZPcQGFG
BjqbMqMVBsfqGqFqGLmF
ZZQbQPddPcwbPnRQltdtQZdnmFNrvfhGrhrWWFNWWtmNFNNW
dJJQccnRPpcbQcMHsSgSMsDMTJSg
WWGBBvPflnWbBWhvhbPvNfnnVCFZmVRVZmVGMVwRLCCCGwVC
gjszgTMrgzgqCRRdmJRjJLVw
grzQHzqczMSzqSHcgQsqPvPlbNblpPhhPPbHvnhp
sJDDNWdnRLTTvqwSFPCmLCCrCq
thzplgfjglflFcbMclpppMfcwPqCZQCmqCwrzCqmQmHSqPqq
MhcpFBMBlhjbBTdnNJWvNvsvBd
czwwghnWWfcfgwfWthfrvVvrjdrdvDDVrbzrLF
RHPPMRpQPRMPPJRjJQsZsrrvvJBDDVDVdFqrBrFdBv
smjMsGZHRsHSmRQNGHPpSTwwttCflwngnChcCtWW
bprrrwrtLDtrWwrQjRDQDbPPVHVmmmmHNWlPlVNPZZlv
hqqhfnBCTfnnhzJwzsqzfPZZMCCVZVHHFvZMFvZmlC
TzhhdJTqJzcBdJJnzjtQrLdjwgLtpbgrLQ
qzQvzzgWSCqtqqGpddGc
jLrZNZhZrNRLHNffhrjNjNdtdZtGcPFwFwpbGwbVpdwC
nHnhrLNCCMHmhHBMhrzvgJvsWSWMWzzWzSlv
RzcbzdRFzbbzbzbFdZFTHMZPhVhVQMLrlrQPhLZlMM
BNGfBvsNttVmMhlMLm
BwGjpllswfjwpcFDWcWcbpdb
SjzpswrLSDjVSpwlmZJBTBdNJLvBNvHQZT
rCcCtbqgCfthggtbGGMqqghqZQvvQTBNJQHQZQTcZTJFZFFd
CggGMtqMfWbbGghPhhbCMtmsSppSspjpmWzjVSWlVrrm
PmWTPThTQWnLWQFl
VNcSVfMbtsddBQNnNpdl
sSjctwjVSzzccjgnTnDTHRDhqjRR
WfMWfCNCjWWHNTccMjRjfRcMbqSwfVwqwsfGGbssrJSrswVw
llLFQLlvlPFnhQBPBZQBqvBwzSzGGhShJVwShmsJbbmzSG
lnPqvQZBFFBnnpgplFvtvHDjTdcTjTMMjCRNCMWgRC
rprFNFFNjNLmMdgcqL
BvzCQQbBQgffsDbvVHMdbcVqmLVqlmqq
JvJCzBDJwnsRnQDszCBnnnQBrjZPjFpgZFTFZRpTrpZFGFtT
wBHQQZHVCcpwDgdZdMsZjvMZFn
GPSzlNlJLfzzzvsWdWLMmFWLMM
NfqGSfrTNzRTqJfRbptQHFQFrwrFHBHw
sNjVMVNVMzPzQgghcMsNzJtjSJtTFDTJtJnnDLjDnL
CHwrdCpvCrwrWdpZqcpFttJSFJTLLHLJfbnbfD
qrlZCwlqZrqqpWdlRqCRqdqcVNsVMzQzmNgNPBsRhVQVVzMs
";
