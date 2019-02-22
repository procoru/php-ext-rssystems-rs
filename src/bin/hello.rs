extern crate rssystems;

use rssystems::zend::_zend_module_entry;

const PHP_RSSYSTEMS_VERSION: &str = "0.1.0";

fn main() -> std::io::Result<()> {
    let rssystems_module_entry = _zend_module_entry::new(
        "rssystems",
        PHP_RSSYSTEMS_VERSION
    );

    println!("PHP Extension Build {:?}", rssystems_module_entry.build_id);

    Ok(())
}

