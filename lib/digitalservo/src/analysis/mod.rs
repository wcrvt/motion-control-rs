pub mod freqeucy_response;

pub struct FrequencyResponse<T> {
    pub freq: T,
    pub re: T,
    pub im: T,
    pub gain: T,
    pub phase: T
}