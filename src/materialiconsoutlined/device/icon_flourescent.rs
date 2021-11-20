
pub struct IconFlourescent {
  props: crate::Props,
}

impl yew::Component for IconFlourescent {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><g><path d="M5,15h14V9H5V15z M7,11h10v2H7V11z"/><rect height="3" width="2" x="11" y="2"/><rect height="2" transform="matrix(0.7046 -0.7096 0.7096 0.7046 1.1814 15.2381)" width="2.54" x="17.62" y="5.2"/><rect height="3" width="2" x="11" y="19"/><polygon points="17.29,17.71 19.08,19.51 20.5,18.09 18.7,16.3"/><rect height="2.53" transform="matrix(0.7071 -0.7071 0.7071 0.7071 -2.8904 5.4222)" width="1.99" x="4.1" y="4.93"/><rect height="2" transform="matrix(0.7096 -0.7046 0.7046 0.7096 -11.1263 8.7897)" width="2.54" x="3.83" y="16.89"/></g></g></svg>
            </svg>
        }
    }
}


