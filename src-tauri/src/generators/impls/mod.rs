pub mod certidoes;
pub mod cnh;
pub mod cnpj;
pub mod company;
pub mod cpf;
pub mod ie;
pub mod person;
pub mod pis;
pub mod renavam;
pub mod rg;
pub mod titulo_eleitor;
pub mod vehicle;

pub use certidoes::{
    CertidaoCasamentoGenerator, CertidaoNascimentoGenerator, CertidaoObitoGenerator,
};
pub use cnh::CnhGenerator;
pub use cnpj::CnpjGenerator;
pub use company::CompanyGenerator;
pub use cpf::CpfGenerator;
pub use ie::InscricaoEstadualGenerator;
pub use person::PersonGenerator;
pub use pis::PisGenerator;
pub use renavam::RenavamGenerator;
pub use rg::RgGenerator;
pub use titulo_eleitor::TituloEleitorGenerator;
pub use vehicle::VehicleGenerator;
