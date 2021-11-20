
pub struct IconTrackChanges {
  props: crate::Props,
}

impl yew::Component for IconTrackChanges {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M18.32 5.68c-.36.36-.39.92-.07 1.32 1.45 1.82 2.21 4.31 1.53 6.92-.79 3.05-3.18 5.33-6.21 5.94C8.47 20.87 4 16.93 4 12c0-4.08 3.05-7.44 7-7.93v2.02c-3.13.53-5.43 3.46-4.93 6.83.39 2.61 2.56 4.71 5.18 5.03C14.89 18.4 18 15.56 18 12c0-1.25-.38-2.4-1.03-3.36-.34-.5-1.07-.53-1.5-.11l-.01.01c-.34.34-.37.87-.11 1.27.6.92.84 2.1.49 3.32-.39 1.37-1.54 2.46-2.94 2.77-2.6.57-4.9-1.39-4.9-3.9 0-1.86 1.28-3.41 3-3.86v2.14c-.6.35-1 .98-1 1.72 0 1.1.9 2 2 2s2-.9 2-2c0-.74-.4-1.38-1-1.72V2.71c0-.39-.32-.71-.71-.71-5.36-.2-9.98 4.06-10.27 9.4-.36 6.55 5.41 11.82 12.01 10.4 3.88-.83 6.88-3.8 7.75-7.67.71-3.16-.2-6.16-1.97-8.37-.37-.47-1.07-.5-1.49-.08z"/></svg>
            </svg>
        }
    }
}


