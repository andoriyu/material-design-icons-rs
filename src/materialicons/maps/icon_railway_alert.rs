
pub struct IconRailwayAlert {
  props: crate::Props,
}

impl yew::Component for IconRailwayAlert {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M23 8a7 7 0 0 0-11.95-4.95A33.8 33.8 0 0 0 9 3c-4.42 0-8 .5-8 4v10.5A3.5 3.5 0 0 0 4.5 21L3 22.5v.5h12v-.5L13.5 21a3.5 3.5 0 0 0 3.5-3.5v-2.58A7 7 0 0 0 23 8zM3 12V7h6.08a6.96 6.96 0 0 0 1.18 5H3zm6 7c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm7.71-6.06l-.2.03L16 13l-.47-.02-.16-.02-.29-.04-.2-.04-.22-.06a1.55 1.55 0 0 1-.23-.07l-.13-.05A4.99 4.99 0 0 1 11.1 7c.04-.19.09-.37.15-.54l.05-.14.15-.38.07-.15.2-.36.07-.12.3-.42.02-.02c.24-.3.52-.57.82-.81l.01-.01.46-.32.03-.02A5.25 5.25 0 0 1 16 3a5 5 0 0 1 .71 9.94zM15 4h2v5h-2zm0 6h2v2h-2z"/></svg>
            </svg>
        }
    }
}


