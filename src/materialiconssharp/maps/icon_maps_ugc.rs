
pub struct IconMapsUgc {
  props: crate::Props,
}

impl yew::Component for IconMapsUgc {
 type Properties = crate::Props;
    type Message = ();

    fn create(props: Self::Properties, _: yew::prelude::ComponentLink<Self>) -> Self
    {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> yew::prelude::ShouldRender
    {
        true
    }

    fn change(&mut self, _: Self::Properties) -> yew::prelude::ShouldRender
    {
        false
    }

    fn view(&self) -> yew::prelude::Html
    {
        yew::prelude::html! {
            <svg
                class=self.props.class.unwrap_or("")
                width=self.props.size.unwrap_or(24).to_string()
                height=self.props.size.unwrap_or(24).to_string()
                viewBox="0 0 24 24"
                fill=self.props.fill.unwrap_or("none")
                stroke=self.props.color.unwrap_or("currentColor")
                stroke-width=self.props.stroke_width.unwrap_or(2).to_string()
                stroke-linecap=self.props.stroke_linecap.unwrap_or("round")
                stroke-linejoin=self.props.stroke_linejoin.unwrap_or("round")
            >
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" fill-rule="evenodd" height="24" width="24" y="0"/><g><path d="M12,4c4.41,0,8,3.59,8,8s-3.59,8-8,8c-1.18,0-2.34-0.26-3.43-0.78c-0.27-0.13-0.56-0.19-0.86-0.19 c-0.19,0-0.38,0.03-0.56,0.08l-3.2,0.94l0.94-3.2c0.14-0.47,0.1-0.98-0.11-1.42C4.26,14.34,4,13.18,4,12C4,7.59,7.59,4,12,4 M12,2 C6.48,2,2,6.48,2,12c0,1.54,0.36,2.98,0.97,4.29L1,23l6.71-1.97C9.02,21.64,10.46,22,12,22c5.52,0,10-4.48,10-10 C22,6.48,17.52,2,12,2L12,2z"/></g><polygon fill-rule="evenodd" points="13,8 11,8 11,11 8,11 8,13 11,13 11,16 13,16 13,13 16,13 16,11 13,11"/></svg>
            </svg>
        }
    }
}


