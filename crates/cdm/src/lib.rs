//! `cdm_core` contains datatypes, functions
//! and logic that are used by the binary implementations
//! of the program

#![warn(clippy::correctness)]
#![warn(clippy::suspicious)]
#![warn(clippy::perf)]
#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
#![warn(clippy::style)]
// restriction lints
#![warn(clippy::arithmetic_side_effects)]
#![warn(clippy::unnecessary_cast)]
#![warn(clippy::cast_lossless)]
#![warn(clippy::cast_possible_truncation)]
#![warn(clippy::cast_possible_wrap)]
#![warn(clippy::cast_precision_loss)]
#![warn(clippy::cast_sign_loss)]
#![warn(clippy::fn_to_numeric_cast)]
#![warn(clippy::fn_to_numeric_cast_with_truncation)]
#![warn(clippy::char_lit_as_u8)]
#![warn(clippy::ptr_as_ptr)]
#![warn(clippy::as_underscore)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::create_dir)]
#![warn(clippy::default_numeric_fallback)]
#![warn(clippy::empty_drop)]
#![warn(clippy::empty_structs_with_brackets)]
#![warn(clippy::exhaustive_enums)]
#![warn(clippy::expect_used)]
#![warn(clippy::filetype_is_file)]
#![warn(clippy::float_cmp_const)]
#![warn(clippy::fn_to_numeric_cast_any)]
#![warn(clippy::format_push_string)]
#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::large_include_file)]
#![warn(clippy::lossy_float_literal)]
#![warn(clippy::map_err_ignore)]
#![warn(clippy::mem_forget)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::mixed_read_write_in_expression)]
#![warn(clippy::mod_module_files)]
#![warn(clippy::multiple_inherent_impl)]
#![warn(clippy::panic_in_result_fn)]
#![warn(clippy::partial_pub_fields)]
#![warn(clippy::print_stderr)]
#![warn(clippy::rc_mutex)]
#![warn(clippy::rest_pat_in_fully_bound_structs)]
#![warn(clippy::same_name_method)]
#![warn(clippy::shadow_unrelated)]
#![warn(clippy::string_add)]
#![warn(clippy::string_to_string)]
#![warn(clippy::todo)]
#![warn(clippy::try_err)]
#![warn(clippy::unimplemented)]
#![warn(clippy::unnecessary_self_imports)]
#![warn(clippy::unseparated_literal_suffix)]
#![warn(clippy::unwrap_in_result)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::verbose_file_reads)]
#![warn(missing_docs)]
/// `config` contains configuration structs that are used for
/// the application binaries
pub mod config;
/// `datatypes` contains all custom types for the
/// `cdm_core` library.
///
/// It also contains functions for parsing data files
/// and the main `Data` type
pub mod datatypes;
