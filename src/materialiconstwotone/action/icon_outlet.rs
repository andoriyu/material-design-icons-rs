
pub struct IconOutlet {
  props: crate::Props,
}

impl yew::Component for IconOutlet {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M12,4c4.41,0,8,3.59,8,8s-3.59,8-8,8s-8-3.59-8-8S7.59,4,12,4 M10,11V8c0-0.55-0.45-1-1-1h0 C8.45,7,8,7.45,8,8v3c0,0.55,0.45,1,1,1h0C9.55,12,10,11.55,10,11z M16,11V8c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v3 c0,0.55,0.45,1,1,1h0C15.55,12,16,11.55,16,11z M14,16c0-1.1-0.9-2-2-2h0c-1.1,0-2,0.9-2,2v2h4V16z" opacity=".3"/><path d="M12,4c4.41,0,8,3.59,8,8s-3.59,8-8,8s-8-3.59-8-8S7.59,4,12,4 M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10 S17.52,2,12,2L12,2z M10,11V8c0-0.55-0.45-1-1-1h0C8.45,7,8,7.45,8,8v3c0,0.55,0.45,1,1,1h0C9.55,12,10,11.55,10,11z M16,11V8 c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v3c0,0.55,0.45,1,1,1h0C15.55,12,16,11.55,16,11z M14,16c0-1.1-0.9-2-2-2h0 c-1.1,0-2,0.9-2,2v2h4V16z"/></svg>
            </svg>
        }
    }
}


