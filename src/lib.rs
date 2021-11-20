#[cfg(feature = "materialicons")]
pub mod materialicons;
#[cfg(feature = "materialiconsoutlined")]
pub mod materialiconsoutlined;
#[cfg(feature = "materialiconsround")]
pub mod materialiconsround;
#[cfg(feature = "materialiconssharp")]
pub mod materialiconssharp;
#[cfg(feature = "materialiconstwotone")]
pub mod materialiconstwotone;


#[derive(yew::Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: Option<&'static str>,
    #[prop_or_default]
    pub size: Option<i64>,
    #[prop_or_default]
    pub color: Option<&'static str>,
    #[prop_or_default]
    pub fill: Option<&'static str>,
    #[prop_or_default]
    pub stroke_width: Option<i64>,
    #[prop_or_default]
    pub stroke_linecap: Option<&'static str>,
    #[prop_or_default]
    pub stroke_linejoin: Option<&'static str>,
}
  