use std::collections::BTreeMap;

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct LabsKey {
    pub size: i32,
    pub strength: i32,
    pub alg: String,
    pub debianFlaw: Option<bool>,
    pub q: Option<i32>,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct LabsCert {
    pub subject: String,
    pub commonNames: Vec<String>,
    pub altNames: Vec<String>,
    pub notBefore: i64,
    pub notAfter: i64,
    pub issuerSubject: String,
    pub sigAlg: String,
    pub issuerLabel: String,
    pub revocationInfo: i32,
    pub crlURIs: Vec<String>,
    pub ocspURIs: Vec<String>,
    pub revocationStatus: i32,
    pub crlRevocationStatus: i32,
    pub ocspRevocationStatus: i32,
    pub sgc: Option<i32>,
    pub validationType: Option<String>,
    pub issues: i32,
    pub sct: Option<bool>,
    pub mustStaple: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct LabsChainCert {
    pub subject: String,
    pub notBefore: i64,
    pub notAfter: i64,
    pub issuerSubject: String,
    pub issuerLabel: String,
    pub sigAlg: String,
    pub issues: i32,
    pub keyAlg: String,
    pub keySize: i32,
    pub keyStrength: i32,
    pub revocationStatus: i32,
    pub crlRevocationStatus: i32,
    pub ocspRevocationStatus: i32,
    pub raw: String,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct LabsChain {
    pub certs: Vec<LabsChainCert>,
    pub issues: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct LabsProtocol {
    pub id: i32,
    pub name: String,
    pub version: String,
    pub v2SuitesDisabled: Option<bool>,
    pub errorMessage: Option<bool>,
    pub q: Option<i32>,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct LabsSimClient {
    id: i32,
    name: String,
    platform: Option<String>,
    version: Option<String>,
    isReference: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct LabsSimulation {
    pub client: LabsSimClient,
    pub errorCode: Option<i32>,
    pub attempts: i32,
    pub protocolId: Option<i32>,
    pub suiteId: Option<i32>,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct LabsSimDetails {
    pub results: Vec<LabsSimulation>,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct LabsSuite {
    pub id: i32,
    pub name: String,
    pub cipherStrength: i32,
    pub dhStrength: Option<i32>,
    pub dhP: Option<i32>,
    pub dhG: Option<i32>,
    pub dhYs: Option<i32>,
    pub ecdhBits: Option<i32>,
    pub ecdhStrength: Option<i32>,
    pub q: Option<i32>,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct LabsSuites {
    pub list: Vec<LabsSuite>,
    pub preference: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct LabsHstsPolicy {
    pub LONG_MAX_AGE: i64,
    pub header: Option<String>,
    pub status: String,
    pub error: Option<String>,
    pub maxAge: Option<i64>,
    pub includeSubDomains: Option<bool>,
    pub preload: Option<bool>,
    pub directives: BTreeMap<String, String>,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct LabsHstsPreload {
    pub source: String,
    pub status: String,
    pub error: Option<String>,
    pub sourceTime: Option<i64>,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct LabsHpkpPin {
    pub hashFunction: String,
    pub value: String,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct LabsHpkpPolicy {
    pub header: Option<String>,
    pub status: String,
    pub error: Option<String>,
    pub maxAge: Option<i64>,
    pub includeSubDomains: Option<bool>,
    pub reportUri: Option<String>,
    pub pins: Vec<LabsHpkpPin>,
    pub matchedPins: Vec<LabsHpkpPin>,
    pub directives: BTreeMap<String, String>,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct DrownHost {
    pub ip: String,
    pub export: bool,
    pub port: i32,
    pub special: bool,
    pub sslv2: bool,
    pub status: String,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct LabsEndpointDetails {
    pub hostStartTime: i64,
    pub key: LabsKey,
    pub cert: LabsCert,
    pub chain: LabsChain,
    pub protocols: Vec<LabsProtocol>,
    pub suites: LabsSuites,
    pub serverSignature: String,
    pub prefixDelegation: Option<bool>,
    pub nonPrefixDelegation: Option<bool>,
    pub vulnBeast: Option<bool>,
    pub renegSupport: i32,
    pub sessionResumption: i32,
    pub compressionMethods: i32,
    pub supportsNpn: Option<bool>,
    pub npnProtocols: Option<String>,
    pub sessionTickets: i32,
    pub ocspStapling: Option<bool>,
    pub staplingRevocationStatus: Option<i32>,
    pub staplingRevocationErrorMessage: Option<String>,
    pub sniRequired: Option<bool>,
    pub httpStatusCode: i32,
    pub httpForwarding: Option<String>,
    pub forwardSecrecy: i32,
    pub supportsRc4: Option<bool>,
    pub rc4WithModern: Option<bool>,
    pub sims: LabsSimDetails,
    pub heartbleed: Option<bool>,
    pub heartbeat: Option<bool>,
    pub openSslCcs: i32,
    pub openSSLLuckyMinus20: i32,
    pub poodle: Option<bool>,
    pub poodleTls: i32,
    pub fallbackScsv: Option<bool>,
    pub freak: Option<bool>,
    pub hasSct: i32,
    pub dhPrimes: Vec<String>,
    pub dhUsesKnownPrimes: i32,
    pub dhYsReuse: Option<bool>,
    pub logjam: Option<bool>,
    pub chaCha20Preference: Option<bool>,
    pub hstsPolicy: LabsHstsPolicy,
    pub hstsPreloads: Vec<LabsHstsPreload>,
    pub hpkpPolicy: LabsHpkpPolicy,
    pub hpkpRoPolicy: LabsHpkpPolicy,
    pub drownHosts: Vec<DrownHost>,
    pub drownErrors: Option<bool>,
    pub drownVulnerable: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct LabsEndpoint {
    pub ipAddress: String,
    pub serverName: Option<String>,
    pub statusMessage: Option<String>,
    pub statusDetailsMessage: Option<String>,
    pub grade: Option<String>,
    pub gradeTrustIgnored: Option<String>,
    pub hasWarnings: Option<bool>,
    pub isExceptional: Option<bool>,
    pub progress: i32,
    pub duration: Option<i32>,
    pub eta: i32,
    pub delegation: i32,
    pub details: Option<LabsEndpointDetails>,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct LabsReport {
    pub host: String,
    pub port: i32,
    pub protocol: String,
    pub isPublic: Option<bool>,
    pub status: String,
    pub statusMessage: Option<String>,
    pub startTime: i64,
    pub testTime: Option<i64>,
    pub engineVersion: String,
    pub criteriaVersion: String,
    pub cacheExpiryTime: Option<i64>,
    pub endpoints: Vec<LabsEndpoint>,
    pub certHostnames: Option<Vec<String>>,
    pub rawJSON: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct LabsResults {
    pub reports: Vec<LabsReport>,
    pub responses: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct LabsInfo {
    pub engineVersion: String,
    pub criteriaVersion: String,
    pub maxAssessments: i32,
    pub currentAssessments: i32,
    pub newAssessmentCoolOff: i64,
    pub messages: Vec<String>,
}
