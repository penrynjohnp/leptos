use crate::ServerMetaContext;
use leptos::{
    attr::{any_attribute::AnyAttribute, NextAttribute},
    component, html,
    reactive::owner::use_context,
    tachys::{
        dom::document,
        html::attribute::Attribute,
        hydration::Cursor,
        view::{
            add_attr::AddAnyAttr, Mountable, Position, PositionState, Render,
            RenderHtml,
        },
    },
    IntoView,
};

/// A component to set metadata on the document’s `<html>` element from
/// within the application.
///
/// This component takes no props, but can take any number of spread attributes
/// following the `{..}` operator.
///
/// ```
/// use leptos::prelude::*;
/// use leptos_meta::*;
///
/// #[component]
/// fn MyApp() -> impl IntoView {
///     provide_meta_context();
///
///     view! {
///       <main>
///         <Html
///           {..}
///           lang="he"
///           dir="rtl"
///           data-theme="dark"
///         />
///       </main>
///     }
/// }
/// ```
#[component]
pub fn Html() -> impl IntoView {
    HtmlView { attributes: () }
}

struct HtmlView<At> {
    attributes: At,
}

struct HtmlViewState<At>
where
    At: Attribute,
{
    attributes: At::State,
}

impl<At> Render for HtmlView<At>
where
    At: Attribute,
{
    type State = HtmlViewState<At>;

    fn build(self) -> Self::State {
        let el = document()
            .document_element()
            .expect("there to be a <html> element");

        let attributes = self.attributes.build(&el);

        HtmlViewState { attributes }
    }

    fn rebuild(self, state: &mut Self::State) {
        self.attributes.rebuild(&mut state.attributes);
    }
}

impl<At> AddAnyAttr for HtmlView<At>
where
    At: Attribute,
{
    type Output<SomeNewAttr: Attribute> =
        HtmlView<<At as NextAttribute>::Output<SomeNewAttr>>;

    fn add_any_attr<NewAttr: Attribute>(
        self,
        attr: NewAttr,
    ) -> Self::Output<NewAttr>
    where
        Self::Output<NewAttr>: RenderHtml,
    {
        HtmlView {
            attributes: self.attributes.add_any_attr(attr),
        }
    }
}

impl<At> RenderHtml for HtmlView<At>
where
    At: Attribute,
{
    type AsyncOutput = HtmlView<At::AsyncOutput>;
    type Owned = HtmlView<At::CloneableOwned>;

    const MIN_LENGTH: usize = At::MIN_LENGTH;

    fn dry_resolve(&mut self) {
        self.attributes.dry_resolve();
    }

    async fn resolve(self) -> Self::AsyncOutput {
        HtmlView {
            attributes: self.attributes.resolve().await,
        }
    }

    fn to_html_with_buf(
        self,
        _buf: &mut String,
        _position: &mut Position,
        _escape: bool,
        _mark_branches: bool,
        extra_attrs: Vec<AnyAttribute>,
    ) {
        if let Some(meta) = use_context::<ServerMetaContext>() {
            let mut buf = String::new();
            _ = html::attributes_to_html(
                (self.attributes, extra_attrs),
                &mut buf,
            );
            if !buf.is_empty() {
                _ = meta.html.send(buf);
            }
        }
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        _cursor: &Cursor,
        _position: &PositionState,
    ) -> Self::State {
        let el = document()
            .document_element()
            .expect("there to be a <html> element");

        let attributes = self.attributes.hydrate::<FROM_SERVER>(&el);

        HtmlViewState { attributes }
    }

    fn into_owned(self) -> Self::Owned {
        HtmlView {
            attributes: self.attributes.into_cloneable_owned(),
        }
    }
}

impl<At> Mountable for HtmlViewState<At>
where
    At: Attribute,
{
    fn unmount(&mut self) {}

    fn mount(
        &mut self,
        _parent: &leptos::tachys::renderer::types::Element,
        _marker: Option<&leptos::tachys::renderer::types::Node>,
    ) {
        // <Html> only sets attributes
        // the <html> tag doesn't need to be mounted anywhere, of course
    }

    fn insert_before_this(&self, _child: &mut dyn Mountable) -> bool {
        false
    }

    fn elements(&self) -> Vec<leptos::tachys::renderer::types::Element> {
        vec![document()
            .document_element()
            .expect("there to be a <html> element")]
    }
}
