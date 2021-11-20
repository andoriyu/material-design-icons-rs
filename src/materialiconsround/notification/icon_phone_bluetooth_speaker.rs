
pub struct IconPhoneBluetoothSpeaker {
  props: crate::Props,
}

impl yew::Component for IconPhoneBluetoothSpeaker {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M19.23 15.26l-2.54-.29c-.61-.07-1.21.14-1.64.57l-1.84 1.84c-2.83-1.44-5.15-3.75-6.59-6.59l1.85-1.85c.43-.43.64-1.03.57-1.64l-.29-2.52c-.12-1.01-.97-1.77-1.99-1.77H5.03c-1.13 0-2.07.94-2 2.07.53 8.54 7.36 15.36 15.89 15.89 1.13.07 2.07-.87 2.07-2v-1.73c.01-1.01-.75-1.86-1.76-1.98zm-2.44-9.25l-2.45 2.45c-.2.2-.2.52 0 .71.2.2.52.2.71 0L17 7.23v3.15c0 .2.12.39.31.47.06.03.13.04.19.04.13 0 .26-.05.36-.15l2.18-2.18c.2-.2.2-.52 0-.71l-1.83-1.83 1.83-1.83c.09-.09.15-.22.15-.36s-.05-.26-.15-.36l-2.18-2.18c-.14-.14-.36-.19-.55-.11-.19.08-.31.26-.31.46v3.15l-1.95-1.95c-.2-.2-.52-.2-.71 0-.2.2-.2.52 0 .71l2.45 2.46zm1.22-3.15l.96.96-.96.96V2.86zm0 4.37l.96.96-.96.96V7.23z"/></svg>
            </svg>
        }
    }
}


