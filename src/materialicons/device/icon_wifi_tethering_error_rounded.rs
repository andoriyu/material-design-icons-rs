
pub struct IconWifiTetheringErrorRounded {
  props: crate::Props,
}

impl yew::Component for IconWifiTetheringErrorRounded {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><g><path d="M12,7c-3.31,0-6,2.69-6,6c0,1.66,0.68,3.15,1.76,4.24l1.42-1.42C8.45,15.1,8,14.11,8,13c0-2.21,1.79-4,4-4s4,1.79,4,4 c0,1.11-0.45,2.1-1.18,2.82l1.42,1.42C17.32,16.15,18,14.66,18,13C18,9.69,15.31,7,12,7z M12,3C6.48,3,2,7.48,2,13 c0,2.76,1.12,5.26,2.93,7.07l1.42-1.42C4.9,17.21,4,15.21,4,13c0-4.42,3.58-8,8-8c2.53,0,4.78,1.17,6.24,3h2.42 C18.93,5.01,15.7,3,12,3z M12,11c-1.1,0-2,0.9-2,2c0,0.55,0.23,1.05,0.59,1.41C10.95,14.77,11.45,15,12,15s1.05-0.23,1.41-0.59 C13.77,14.05,14,13.55,14,13C14,11.9,13.1,11,12,11z M20,10h2v6h-2V10z M20,18h2v2h-2V18z"/></g></g></svg>
            </svg>
        }
    }
}


