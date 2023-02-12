use std::{collections::BTreeMap, path::PathBuf};

use openssl::ssl::{SslContext, SslContextBuilder, SslFiletype, SslMethod, SslVersion};

use crate::{Result, TlsConfig};

pub fn build_ssl_context(pkf: PathBuf, cf: PathBuf) -> Result<SslContext> {
    let mut builder = SslContextBuilder::new(SslMethod::tls())?;

    builder.set_private_key_file(pkf, SslFiletype::PEM)?;
    builder.set_certificate_chain_file(cf)?;
    builder.set_min_proto_version(Some(SslVersion::TLS1))?;

    let ctx = builder.build();

    Ok(ctx)
}

pub fn build_ssl_context_map(configs: Vec<TlsConfig>) -> Result<BTreeMap<String, SslContext>> {
    let mut res = BTreeMap::new();

    for tls in configs {
        let ctx = build_ssl_context(tls.private_key_file, tls.certificate_file)?;
        res.insert(tls.sni, ctx);
    }
    Ok(res)
}
