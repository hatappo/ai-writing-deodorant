use deo_core::process_text;
use leptos::*;
use wasm_bindgen::JsCast;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Lang {
    En,
    Ja,
}

impl Lang {
    fn toggle(self) -> Self {
        match self {
            Lang::En => Lang::Ja,
            Lang::Ja => Lang::En,
        }
    }
}

#[derive(Clone)]
struct Texts {
    title: &'static str,
    subtitle: &'static str,
    input: &'static str,
    output: &'static str,
    remove_emoji: &'static str,
    upload_file: &'static str,
    placeholder: &'static str,
    copy: &'static str,
    copied: &'static str,
    switch_lang: &'static str,
    note: &'static str,
}

impl Texts {
    fn en() -> Self {
        Self {
            title: "AI Writing Deodorant",
            subtitle: "Remove AI-like formatting from text",
            input: "Input",
            output: "Output",
            remove_emoji: "Remove emoji",
            upload_file: "Upload file",
            placeholder: "Paste your AI-generated text here...",
            copy: "Copy",
            copied: "Copied!",
            switch_lang: "日本語",
            note: "This tool just removes \"**\" (and optionally emoji). That's it.",
        }
    }

    fn ja() -> Self {
        Self {
            title: "文章からAIっぽさをなくす",
            subtitle: "AI臭のデオドラント",
            input: "入力",
            output: "出力",
            remove_emoji: "絵文字も除去",
            upload_file: "ファイルを選択",
            placeholder: "AI生成テキストをここに貼り付け...",
            copy: "コピー",
            copied: "コピー完了",
            switch_lang: "English",
            note: "このツールは \"**\" を消すだけです（オプションで絵文字も）。それだけ。",
        }
    }

    fn get(lang: Lang) -> Self {
        match lang {
            Lang::En => Self::en(),
            Lang::Ja => Self::ja(),
        }
    }
}

fn auto_resize_textarea(textarea: &web_sys::HtmlTextAreaElement) {
    textarea.style().set_property("height", "auto").unwrap();
    let scroll_height = textarea.scroll_height();
    let min_height = 300;
    let height = scroll_height.max(min_height);
    textarea
        .style()
        .set_property("height", &format!("{}px", height))
        .unwrap();
}

#[component]
fn App() -> impl IntoView {
    let (lang, set_lang) = create_signal(Lang::En);
    let (input, set_input) = create_signal(String::new());
    let (remove_emoji, set_remove_emoji) = create_signal(false);
    let textarea_ref = create_node_ref::<html::Textarea>();

    let t = move || Texts::get(lang.get());
    let output = move || process_text(&input.get(), remove_emoji.get());

    create_effect(move |_| {
        let _ = input.get();
        if let Some(textarea) = textarea_ref.get() {
            let el: &web_sys::HtmlTextAreaElement = &textarea;
            auto_resize_textarea(el);
        }
    });

    view! {
        <main class="container">
            <header class="header">
                <div class="header-content">
                    <h1>{move || t().title}</h1>
                    <p class="subtitle">{move || t().subtitle}</p>
                </div>
                <button class="lang-btn" on:click=move |_| set_lang.update(|l| *l = l.toggle())>
                    {move || t().switch_lang}
                </button>
            </header>

            <div class="panels">
                <div class="panel">
                    <div class="panel-header">
                        <div class="header-left">
                            <span>{move || t().input}</span>
                            <FileUpload set_input=set_input t=t />
                        </div>
                        <label class="toggle">
                            <input
                                type="checkbox"
                                on:change=move |e| set_remove_emoji.set(event_target_checked(&e))
                            />
                            <span>{move || t().remove_emoji}</span>
                        </label>
                    </div>
                    <textarea
                        node_ref=textarea_ref
                        placeholder=move || t().placeholder
                        on:input=move |e| set_input.set(event_target_value(&e))
                        prop:value=move || input.get()
                    />
                </div>

                <div class="panel">
                    <div class="panel-header">
                        <span>{move || t().output}</span>
                        <CopyButton text=output t=t />
                    </div>
                    <div class="output">
                        {output}
                    </div>
                </div>
            </div>

            <footer class="footer">
                <p>{move || t().note}</p>
            </footer>
        </main>
    }
}

#[component]
fn FileUpload<F>(set_input: WriteSignal<String>, t: F) -> impl IntoView
where
    F: Fn() -> Texts + 'static + Copy,
{
    let on_file_change = move |e: ev::Event| {
        let input: web_sys::HtmlInputElement = event_target(&e);
        if let Some(files) = input.files() {
            if let Some(file) = files.get(0) {
                let reader = web_sys::FileReader::new().unwrap();
                let reader_clone = reader.clone();

                let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
                    if let Ok(result) = reader_clone.result() {
                        if let Some(text) = result.as_string() {
                            set_input.set(text);
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                onload.forget();
                reader.read_as_text(&file).unwrap();
            }
        }
    };

    view! {
        <div class="file-upload">
            <label class="file-label">
                <input type="file" accept=".txt,.md" on:change=on_file_change />
                <span>{move || t().upload_file}</span>
            </label>
        </div>
    }
}

#[component]
fn CopyButton<F, T>(text: F, t: T) -> impl IntoView
where
    F: Fn() -> String + 'static,
    T: Fn() -> Texts + 'static + Copy,
{
    let (copied, set_copied) = create_signal(false);

    let on_click = move |_| {
        let text_to_copy = text();
        let window = web_sys::window().unwrap();
        let navigator = window.navigator();
        let clipboard = navigator.clipboard();

        wasm_bindgen_futures::spawn_local(async move {
            let promise = clipboard.write_text(&text_to_copy);
            let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
            set_copied.set(true);

            gloo_timers::callback::Timeout::new(2000, move || {
                set_copied.set(false);
            })
            .forget();
        });
    };

    view! {
        <button class="copy-btn" on:click=on_click>
            {move || if copied.get() { t().copied } else { t().copy }}
        </button>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
