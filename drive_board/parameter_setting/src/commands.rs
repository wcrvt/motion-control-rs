pub struct DigitalServoCommandSet {}

pub const TX_DELIMITER: &str = "\r\n";
pub const RX_DELIMITER: &str = "\r\n";

impl DigitalServoCommandSet {
    pub fn parameters_for_mn501(channel: u8) -> Vec<String> {
        vec![
            /* Update System parameters  */
            format!("_sendresponse {channel} 129 SamplingFreq double 80000{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 SamplingFreq{TX_DELIMITER}"),

            /* Update Motor parameters  */
            /* Update Motor(electric) parameters  */
            format!("_sendresponse {channel} 129 R double 0.05{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 R{TX_DELIMITER}"),
            format!("_sendresponse {channel} 129 Ld double 0.030e-3{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 Ld{TX_DELIMITER}"),
            format!("_sendresponse {channel} 129 Lq double 0.030e-3{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 Lq{TX_DELIMITER}"),
            format!("_sendresponse {channel} 129 Phi double 2.21e-3{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 Phi{TX_DELIMITER}"),

            /* Update Motor(mechanical) parameters  */
            format!("_sendresponse {channel} 129 Poles double 28{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 Poles{TX_DELIMITER}"),
            format!("_sendresponse {channel} 129 Jm double 5.8e-6{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 Jm{TX_DELIMITER}"),

            /* HFI parameters  */
            format!("_sendresponse {channel} 129 Nh double 4{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 Nh{TX_DELIMITER}"),
            format!("_sendresponse {channel} 129 K double 1{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 K{TX_DELIMITER}"),
            format!("_sendresponse {channel} 129 Vh double 3{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 Vh{TX_DELIMITER}"),
            format!("_sendresponse {channel} 129 ADCTrigMod double 0{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 ADCTrigMod{TX_DELIMITER}"),
            format!("_sendresponse {channel} 129 ADCTrigTime double 550e-9{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 ADCTrigTime{TX_DELIMITER}"),

            /* Pulse parameters  */
            format!("_sendresponse {channel} 129 PulseTargetCurrent double 6.5{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 PulseTargetCurrent{TX_DELIMITER}"),
            format!("_sendresponse {channel} 129 PulseSNLowerErrorValue double 0.000001{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 PulseSNLowerErrorValue{TX_DELIMITER}"),

            /* Hybrid parameters  */
            format!("_sendresponse {channel} 129 SwitchOmegaL double 200{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 SwitchOmegaL{TX_DELIMITER}"),
            format!("_sendresponse {channel} 129 SwitchOmegaH double 350{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 SwitchOmegaH{TX_DELIMITER}"),
            format!("_sendresponse {channel} 129 SwitchOmegaHFIVoltageOffset double 150{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 SwitchOmegaHFIVoltageOffset{TX_DELIMITER}"),

            /* CurrentCtrl parameters  */
            format!("_sendresponse {channel} 129 CurrentCtrlBandwidth double 2500{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 CurrentCtrlBandwidth{TX_DELIMITER}"),
            format!("_sendresponse {channel} 129 CurrentLimit double 5{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 CurrentLimit{TX_DELIMITER}"),

            /* PLL parameters  */
            format!("_sendresponse {channel} 129 PLLBandwidth double 1000{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 PLLBandwidth{TX_DELIMITER}"),
            format!("_sendresponse {channel} 129 PLLOmegaLPFBandwidth double 250{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 PLLOmegaLPFBandwidth{TX_DELIMITER}"),

            /* SpeedCtrl parameters  */
            format!("_sendresponse {channel} 129 SpeedCtrlBandwidth double 75{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 SpeedCtrlBandwidth{TX_DELIMITER}"),

            /* PositionCtrl parameters  */
            format!("_sendresponse {channel} 129 PositionCtrlBandwidth double 10{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 PositionCtrlBandwidth{TX_DELIMITER}"),

            /* DOB parameters  */
            format!("_sendresponse {channel} 129 DOBBandwidth double 100{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 DOBBandwidth{TX_DELIMITER}"),

            format!("_sendresponse {channel} 129 Cf double 1{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 Cf{TX_DELIMITER}"),

            format!("_sendresponse {channel} 129 DriveMode double 2{TX_DELIMITER}"),
            format!("_sendrequest {channel} 128 DriveMode{TX_DELIMITER}"),

            format!("_sendresponse {channel} 129 cmdval double 0{TX_DELIMITER}"),
            format!("_sendresponse {channel} 129 drive double 0{TX_DELIMITER}"),
        ]
    }
}