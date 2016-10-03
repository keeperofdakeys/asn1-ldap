#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[macro_use]
extern crate asn1_cereal;

use asn1_cereal::OctetString;

struct LDAPMessage {
  messageID: i32,
  protocolOp: ProtocolOp,
  controls: Option<Controls>,
}

ber_sequence!(
  LDAPMessage,
  "LDAPMessage",
  messageID;
  protocolOp;
  controls ([0] OPTIONAL);
);

enum ProtocolOp {
  bindRequest(BindRequest),
}

ber_choice!(
  ProtocolOp,
  "CHOICE",
  bindRequest
);

struct LdapDN(OctetString);
ber_alias!(LdapDN ::= OctetString, "LdapDN");

struct LdapOID(OctetString);
ber_alias!(LdapOID ::= OctetString, "LdapOID");

struct Controls(Vec<Control>);

ber_alias!(Controls ::= Vec<Control>, "Controls");

struct Control {
  controlType: LdapOID,
  criticality: bool,
  controlValue: Option<OctetString>,
}

ber_sequence!(
  Control,
  "Control",
  controlType;
  criticality (DEFAULT false);
  controlValue ([0] OPTIONAL);
);

struct BindRequest {
  version: u32,
  name: LdapDN,
  authentication: AuthenticationChoice,
}

ber_sequence!(
  BindRequest,
  [APPLICATION 0],
  "BindRequest",
  version;
  name;
  authentication;
);

enum AuthenticationChoice {
  simple(OctetString),
}

ber_choice!(
  AuthenticationChoice,
  "AuthenticationChoice",
  simple
);

