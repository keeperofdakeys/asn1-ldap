#[macro_use]
extern crate asn1_cereal;

use asn1_cereal::tag;

struct LdapDN(String);

struct BindRequest {
  version: u32,
  name: LdapDN,
  //authentication: AuthenticationChoice,
}

ber_alias!(LdapDN, String);

asn1_info!(BindRequest, tag::Class::Application, 0, true, "Bind Request");
ber_sequence_serialize!(BindRequest, version, name);
ber_sequence_deserialize!(BindRequest, version, name);
