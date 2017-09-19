#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate asn1_cereal;
#[macro_use]
extern crate asn1_cereal_derive;
#[macro_use]
extern crate log;
extern crate env_logger;

use asn1_cereal::OctetString;

#[derive(Asn1Info, BerSerialize, BerDeserialize)]
#[asn1(log)]
struct LDAPMessage {
  messageID: i32,
  protocolOp: ProtocolOp,
  // controls: Option<Controls>,
}

#[derive(Asn1Info, BerSerialize, BerDeserialize)]
#[asn1(log)]
enum ProtocolOp {
  bindRequest(BindRequest),
}

#[derive(Asn1Info, BerSerialize, BerDeserialize)]
#[asn1(log)]
struct LdapDN(OctetString);

#[derive(Asn1Info, BerSerialize, BerDeserialize)]
#[asn1(log)]
struct LdapOID(OctetString);

#[derive(Asn1Info, BerSerialize, BerDeserialize)]
#[asn1(log)]
struct Controls(Vec<Control>);

#[derive(Asn1Info, BerSerialize, BerDeserialize)]
#[asn1(log)]
struct Control {
  controlType: LdapOID,
  criticality: bool,
  // controlValue: Option<OctetString>,
}

#[derive(Asn1Info, BerSerialize, BerDeserialize)]
#[asn1(tag="[APPLICATION 8]", log)]
struct BindRequest {
  version: u32,
  name: LdapDN,
  authentication: AuthenticationChoice,
}

#[derive(Asn1Info, BerSerialize, BerDeserialize)]
#[asn1(log)]
enum AuthenticationChoice {
  simple(OctetString),
}
