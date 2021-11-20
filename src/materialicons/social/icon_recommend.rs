
pub struct IconRecommend {
  props: crate::Props,
}

impl yew::Component for IconRecommend {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0z" fill="none"/>{ "<title>ic_recommend_24px</title>" }<path d="M12 2a10 10 0 1 0 10 10A10 10 0 0 0 12 2zm6 9.8a.9.9 0 0 1-.1.5l-2.1 4.9a1.34 1.34 0 0 1-1.3.8H9a2 2 0 0 1-2-2v-5a1.28 1.28 0 0 1 .4-1L12 5l.69.69a1.08 1.08 0 0 1 .3.7v.2L12.41 10H17a1 1 0 0 1 1 1z"/></svg>
            </svg>
        }
    }
}


