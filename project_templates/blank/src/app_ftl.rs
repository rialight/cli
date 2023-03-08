use rialight::intl::ftl::{Ftl, FtlOptions, FtlOptionsForAssets, FtlLoadMethod};
use rialight::util::{hashmap};
use std::sync::Arc;

pub fn create() -> Arc<Ftl> {
    Arc::new({
        let mut app_ftl = Ftl::new(
            FtlOptions::new()
                // specify supported locales.
                // the form in which the locale identifier appears here
                // is a post-component for the assets "src" path. 
                // for example: "path/to/res/lang/en-US"
                .supported_locales(vec!["en"])
                .default_locale("en")
                // .fallbacks(hashmap! {
                //     "xx" => vec!["xy"],
                // })
                .assets(FtlOptionsForAssets::new()
                    .source("app://res/lang")
                    .files(vec!["_"])
                    // "clean_unused" indicates whether to clean previous unused locale data. 
                    .clean_unused(true)
                    // specify FtlLoadMethod::FileSystem or FtlLoadMethod::Http
                    .load_method(FtlLoadMethod::FileSystem)))
        ;
        app_ftl.initialize_locale(|_locale, _bundle| {
            //
        });
        app_ftl
    })
}