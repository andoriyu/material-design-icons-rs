
pub struct IconResetTv {
  props: crate::Props,
}

impl yew::Component for IconResetTv {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><path d="M20,10h-7.01V8.21c0-0.45-0.54-0.67-0.85-0.35l-2.78,2.79c-0.19,0.2-0.19,0.51,0,0.71l2.78,2.79 c0.31,0.32,0.85,0.09,0.85-0.35V12H20v5H4V5h16v2c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1l0-2c0-1.1-0.9-2-2-2H4C2.9,3,2,3.9,2,5v12 c0,1.1,0.9,2,2,2h4v1c0,0.55,0.45,1,1,1h6c0.55,0,1-0.45,1-1v-1h4c1.1,0,2-0.9,2-2v-5C22,10.9,21.1,10,20,10z"/></g></svg>
            </svg>
        }
    }
}


