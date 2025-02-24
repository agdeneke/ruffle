use super::AvmAtom;
use gc_arena::Collect;

macro_rules! define_common_strings {
    (
        $ascii:ident: <ASCII>,
        $($field:ident: $str:literal,)*
    ) => {
        #[allow(non_snake_case)]
        #[derive(Collect)]
        #[collect(no_drop)]
        pub struct CommonStrings<'gc> {
            pub $ascii: [AvmAtom<'gc>; ASCII_CHARS_LEN],

            $(
                pub $field: AvmAtom<'gc>,
            )*
        }

        impl<'gc> CommonStrings<'gc> {
            pub(super) fn new(mut intern_from_static: impl FnMut(&'static [u8]) -> AvmAtom<'gc>) -> Self {
                Self {
                    $ascii: std::array::from_fn(|i| {
                        let c = &ASCII_CHARS[i];
                        intern_from_static(std::slice::from_ref(c))
                    }),
                    $($field: intern_from_static($str)),*
                }
            }
        }
    };
}

const ASCII_CHARS_LEN: usize = 0x80;
static ASCII_CHARS: [u8; ASCII_CHARS_LEN] = {
    let mut chs = [0; ASCII_CHARS_LEN];
    let mut i = 0;
    while i < chs.len() {
        chs[i] = i as u8;
        i += 1;
    }
    chs
};

define_common_strings! {
    ascii_chars: <ASCII>,

    // Alphanumeric strings, in alphabetical order.
    // The field name should always be the string prefixed with `str_`.
    str_: b"",
    str___constructor__: b"__constructor__",
    str___proto__: b"__proto__",
    str___resolve: b"__resolve",
    str__bytesLoaded: b"_bytesLoaded",
    str__bytesTotal: b"_bytesTotal",
    str_access: b"access",
    str_accessors: b"accessors",
    str_advanced: b"advanced",
    str_alphaMultiplier: b"alphaMultiplier",
    str_alphaOffset: b"alphaOffset",
    str_always: b"always",
    str_ascent: b"ascent",
    str_asyncError: b"asyncError",
    str_auto: b"auto",
    str_bases: b"bases",
    str_block: b"block",
    str_blueMultiplier: b"blueMultiplier",
    str_blueOffset: b"blueOffset",
    str_bold: b"bold",
    str_boldItalic: b"boldItalic",
    str_boolean: b"boolean",
    str_builtInItems: b"builtInItems",
    str_callee: b"callee",
    str_caption: b"caption",
    str_center: b"center",
    str_clamp: b"clamp",
    str_click: b"click",
    str_color: b"color",
    str_constructor: b"constructor",
    str_customItems: b"customItems",
    str_declaredBy: b"declaredBy",
    str_decode: b"decode",
    str_descent: b"descent",
    str_device: b"device",
    str_doubleClick: b"doubleClick",
    str_dynamic: b"dynamic",
    str_embedded: b"embedded",
    str_embeddedCFF: b"embeddedCFF",
    str_enabled: b"enabled",
    str_false: b"false",
    str_forward_back: b"forward_back",
    str_full: b"full",
    str_function: b"function",
    str_greenMultiplier: b"greenMultiplier",
    str_greenOffset: b"greenOffset",
    str_height: b"height",
    str_httpStatus: b"httpStatus",
    str_ignore: b"ignore",
    str_ignoreWhite: b"ignoreWhite",
    str_Infinity: b"Infinity",
    str_inline: b"inline",
    str_inner: b"inner",
    str_input: b"input",
    str_interfaces: b"interfaces",
    str_ioError: b"ioError",
    str_isDynamic: b"isDynamic",
    str_isFinal: b"isFinal",
    str_isStatic: b"isStatic",
    str_italic: b"italic",
    str_justify: b"justify",
    str_key: b"key",
    str_keyDown: b"keyDown",
    str_keyUp: b"keyUp",
    str_left: b"left",
    str_length: b"length",
    str_loaded: b"loaded",
    str_localName: b"localName",
    str_loop: b"loop",
    str_matrixType: b"matrixType",
    str_menu: b"menu",
    str_metadata: b"metadata",
    str_methods: b"methods",
    str_middleClick: b"middleClick",
    str_middleMouseDown: b"middleMouseDown",
    str_middleMouseUp: b"middleMouseUp",
    str_mouseDown: b"mouseDown",
    str_mouseMove: b"mouseMove",
    str_mouseOut: b"mouseOut",
    str_mouseOver: b"mouseOver",
    str_mouseUp: b"mouseUp",
    str_mouseWheel: b"mouseWheel",
    str_movieclip: b"movieclip",
    str_name: b"name",
    str_NaN: b"NaN",
    str_netStatus: b"netStatus",
    str_never: b"never",
    str_none: b"none",
    str_normal: b"normal",
    str_null: b"null",
    str_number: b"number",
    str_object: b"object",
    str_onCancel: b"onCancel",
    str_onClose: b"onClose",
    str_onComplete: b"onComplete",
    str_onConnect: b"onConnect",
    str_onData: b"onData",
    str_onIOError: b"onIOError",
    str_onHTTPError: b"onHTTPError",
    str_onHTTPStatus: b"onHTTPStatus",
    str_onLoad: b"onLoad",
    str_onOpen: b"onOpen",
    str_onProgress: b"onProgress",
    str_onResult: b"onResult",
    str_onSelect: b"onSelect",
    str_onStatus: b"onStatus",
    str_optional: b"optional",
    str_outer: b"outer",
    str_parameters: b"parameters",
    str_pixel: b"pixel",
    str_play: b"play",
    str_prefix: b"prefix",
    str_print: b"print",
    str_prototype: b"prototype",
    str_quality: b"quality",
    str_readonly: b"readonly",
    str_readwrite: b"readwrite",
    str_redMultiplier: b"redMultiplier",
    str_redOffset: b"redOffset",
    str_regular: b"regular",
    str_releaseOutside: b"releaseOutside",
    str_returnType: b"returnType",
    str_rewind: b"rewind",
    str_right: b"right",
    str_rightClick: b"rightClick",
    str_rightMouseDown: b"rightMouseDown",
    str_rightMouseUp: b"rightMouseUp",
    str_rollOut: b"rollOut",
    str_rollOver: b"rollOver",
    str_save: b"save",
    str_separatorBefore: b"separatorBefore",
    str_status: b"status",
    str_string: b"string",
    str_subpixel: b"subpixel",
    str_textFieldHeight: b"textFieldHeight",
    str_textFieldWidth: b"textFieldWidth",
    str_toJSON: b"toJSON",
    str_toString: b"toString",
    str_traits: b"traits",
    str_true: b"true",
    str_tx: b"tx",
    str_ty: b"ty",
    str_type: b"type",
    str_undefined: b"undefined",
    str_uri: b"uri",
    str_value: b"value",
    str_valueOf: b"valueOf",
    str_variables: b"variables",
    str_visible: b"visible",
    str_width: b"width",
    str_wrap: b"wrap",
    str_writeonly: b"writeonly",
    str_xMax: b"xMax",
    str_xMin: b"xMin",
    str_xml: b"xml",
    str_yMax: b"yMax",
    str_yMin: b"yMin",
    str_zoom: b"zoom",
}
