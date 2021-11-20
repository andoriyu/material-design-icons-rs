
pub struct IconNewReleases {
  props: crate::Props,
}

impl yew::Component for IconNewReleases {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M22.42 11.34l-1.86-2.12.26-2.81c.05-.5-.29-.96-.77-1.07l-2.76-.63-1.44-2.43c-.26-.43-.79-.61-1.25-.41L12 3 9.41 1.89c-.46-.2-1-.02-1.25.41L6.71 4.72l-2.75.62c-.49.11-.83.56-.78 1.07l.26 2.8-1.86 2.13c-.33.38-.33.94 0 1.32l1.86 2.12-.26 2.82c-.05.5.29.96.77 1.07l2.76.63 1.44 2.42c.26.43.79.61 1.26.41L12 21l2.59 1.11c.46.2 1 .02 1.25-.41l1.44-2.43 2.76-.63c.49-.11.82-.57.77-1.07l-.26-2.81 1.86-2.12c.34-.36.34-.92.01-1.3zM13 17h-2v-2h2v2zm-1-4c-.55 0-1-.45-1-1V8c0-.55.45-1 1-1s1 .45 1 1v4c0 .55-.45 1-1 1z"/></svg>
            </svg>
        }
    }
}


