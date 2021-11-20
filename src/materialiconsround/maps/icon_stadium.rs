
pub struct IconStadium {
  props: crate::Props,
}

impl yew::Component for IconStadium {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M6.11,5.45L3.72,6.64C3.39,6.8,3,6.56,3,6.19V3.81C3,3.44,3.39,3.2,3.72,3.36l2.38,1.19C6.47,4.74,6.47,5.26,6.11,5.45z M18,3.81v2.38c0,0.37,0.39,0.61,0.72,0.45l2.38-1.19c0.37-0.18,0.37-0.71,0-0.89l-2.38-1.19C18.39,3.2,18,3.44,18,3.81z M11,2.81 v2.38c0,0.37,0.39,0.61,0.72,0.45l2.38-1.19c0.37-0.18,0.37-0.71,0-0.89l-2.38-1.19C11.39,2.2,11,2.44,11,2.81z M5,10.04 C6.38,10.53,8.77,11,12,11s5.62-0.47,7-0.96C19,9.86,16.22,9,12,9S5,9.86,5,10.04z M14,17h-4c-0.55,0-1,0.45-1,1l0,3.88 C4.94,21.49,2,20.34,2,19v-9c0-1.66,4.48-3,10-3s10,1.34,10,3v9c0,1.34-2.94,2.48-7,2.87L15,18C15,17.45,14.55,17,14,17z"/></g></svg>
            </svg>
        }
    }
}


