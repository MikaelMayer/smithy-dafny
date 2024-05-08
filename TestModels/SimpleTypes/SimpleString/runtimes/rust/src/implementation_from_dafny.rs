#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]
pub use dafny_standard_library::implementation_from_dafny::*;


pub mod r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes {
  /* datatype DafnyCallEvent<I, O> = DafnyCallEvent(input: I, output: O) */
  #[derive(PartialEq, Clone)]
  pub enum DafnyCallEvent<I: ::dafny_runtime::DafnyType, O: ::dafny_runtime::DafnyType> {
    DafnyCallEvent {
      input: I,
      output: O
    },
    _PhantomVariant(::std::marker::PhantomData<I>,
      ::std::marker::PhantomData<O>)
  }

  impl<I: ::dafny_runtime::DafnyType, O: ::dafny_runtime::DafnyType> DafnyCallEvent<I, O> {
    pub fn input(&self) -> &I {
      match self {
        DafnyCallEvent::DafnyCallEvent { input, output, } => input,
        DafnyCallEvent::_PhantomVariant(..) => panic!(),
      }
    }
    pub fn output(&self) -> &O {
      match self {
        DafnyCallEvent::DafnyCallEvent { input, output, } => output,
        DafnyCallEvent::_PhantomVariant(..) => panic!(),
      }
    }
  }

  impl<I: ::dafny_runtime::DafnyType, O: ::dafny_runtime::DafnyType> ::std::fmt::Debug
    for DafnyCallEvent<I, O> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::fmt::Result {
      ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
    }
  }

  impl<I: ::dafny_runtime::DafnyType, O: ::dafny_runtime::DafnyType> ::dafny_runtime::DafnyPrint
    for DafnyCallEvent<I, O> {
    fn fmt_print(&self, _formatter: &mut ::std::fmt::Formatter, _in_seq: bool) -> std::fmt::Result {
      match self {
        DafnyCallEvent::DafnyCallEvent { input, output, } => {
          write!(_formatter, "r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes.DafnyCallEvent.DafnyCallEvent(")?;
          ::dafny_runtime::DafnyPrint::fmt_print(input, _formatter, false)?;
          write!(_formatter, ", ")?;
          ::dafny_runtime::DafnyPrint::fmt_print(output, _formatter, false)?;
          write!(_formatter, ")")?;
          Ok(())
        },
        DafnyCallEvent::_PhantomVariant(..) => {panic!()},
      }
    }
  }

  impl<I: ::dafny_runtime::DafnyType + Eq, O: ::dafny_runtime::DafnyType + Eq> Eq
    for DafnyCallEvent<I, O> {}

  impl<I: ::dafny_runtime::DafnyType + ::std::hash::Hash, O: ::dafny_runtime::DafnyType + ::std::hash::Hash> ::std::hash::Hash
    for DafnyCallEvent<I, O> {
    fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
      match self {
        DafnyCallEvent::DafnyCallEvent { input, output, } => {
          input.hash(_state);
          output.hash(_state)
        },
        DafnyCallEvent::_PhantomVariant(..) => {panic!()},
      }
    }
  }

  impl<I: ::dafny_runtime::DafnyType + ::std::default::Default, O: ::dafny_runtime::DafnyType + ::std::default::Default> ::std::default::Default
    for DafnyCallEvent<I, O> {
    fn default() -> DafnyCallEvent<I, O> {
      DafnyCallEvent::DafnyCallEvent {
        input: ::std::default::Default::default(),
        output: ::std::default::Default::default()
      }
    }
  }

  impl<I: ::dafny_runtime::DafnyType + ::std::default::Default, O: ::dafny_runtime::DafnyType + ::std::default::Default> ::std::convert::AsRef<DafnyCallEvent<I, O>>
    for &DafnyCallEvent<I, O> {
    fn as_ref(&self) -> Self {
      self
    }
  }

  /*
   datatype GetStringInput = | GetStringInput (
    nameonly value: Option<string> := Option.None
  )
  */
  #[derive(PartialEq, Clone)]
  pub enum GetStringInput {
    GetStringInput {
      value: ::std::rc::Rc<super::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>
    }
  }

  impl GetStringInput {
    pub fn value(&self) -> &::std::rc::Rc<super::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>> {
      match self {
        GetStringInput::GetStringInput { value, } => value,
      }
    }
  }

  impl ::std::fmt::Debug
    for GetStringInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::fmt::Result {
      ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
    }
  }

  impl ::dafny_runtime::DafnyPrint
    for GetStringInput {
    fn fmt_print(&self, _formatter: &mut ::std::fmt::Formatter, _in_seq: bool) -> std::fmt::Result {
      match self {
        GetStringInput::GetStringInput { value, } => {
          write!(_formatter, "r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes.GetStringInput.GetStringInput(")?;
          ::dafny_runtime::DafnyPrint::fmt_print(value, _formatter, false)?;
          write!(_formatter, ")")?;
          Ok(())
        },
      }
    }
  }

  impl Eq
    for GetStringInput {}

  impl ::std::hash::Hash
    for GetStringInput {
    fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
      match self {
        GetStringInput::GetStringInput { value, } => {
          value.hash(_state)
        },
      }
    }
  }

  impl ::std::default::Default
    for GetStringInput {
    fn default() -> GetStringInput {
      GetStringInput::GetStringInput {
        value: ::std::default::Default::default()
      }
    }
  }

  impl ::std::convert::AsRef<GetStringInput>
    for &GetStringInput {
    fn as_ref(&self) -> Self {
      self
    }
  }

  
  /*
  datatype GetStringOutput = | GetStringOutput (
  nameonly value: Option<string> := Option.None
  ) */
  #[derive(PartialEq, Clone)]
  pub enum GetStringOutput {
    GetStringOutput {
      value: ::std::rc::Rc<super::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>
    }
  }

  impl GetStringOutput {
    pub fn value(&self) -> &::std::rc::Rc<super::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>> {
      match self {
        GetStringOutput::GetStringOutput { value, } => value,
      }
    }
  }

  impl ::std::fmt::Debug
    for GetStringOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::fmt::Result {
      ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
    }
  }

  impl ::dafny_runtime::DafnyPrint
    for GetStringOutput {
    fn fmt_print(&self, _formatter: &mut ::std::fmt::Formatter, _in_seq: bool) -> std::fmt::Result {
      match self {
        GetStringOutput::GetStringOutput { value, } => {
          write!(_formatter, "r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes.GetStringOutput.GetStringOutput(")?;
          ::dafny_runtime::DafnyPrint::fmt_print(value, _formatter, false)?;
          write!(_formatter, ")")?;
          Ok(())
        },
      }
    }
  }

  impl Eq
    for GetStringOutput {}

  impl ::std::hash::Hash
    for GetStringOutput {
    fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
      match self {
        GetStringOutput::GetStringOutput { value, } => {
          value.hash(_state)
        },
      }
    }
  }

  impl ::std::default::Default
    for GetStringOutput {
    fn default() -> GetStringOutput {
      GetStringOutput::GetStringOutput {
        value: ::std::default::Default::default()
      }
    }
  }

  impl ::std::convert::AsRef<GetStringOutput>
    for &GetStringOutput {
    fn as_ref(&self) -> Self {
      self
    }
  }

  /*
  datatype SimpleStringConfig = | SimpleStringConfig (
  ) */

  #[derive(PartialEq, Clone)]
  pub enum SimpleStringConfig {
    SimpleStringConfig {}
  }

  impl SimpleStringConfig {}

  impl ::std::fmt::Debug
    for SimpleStringConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::fmt::Result {
      ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
    }
  }

  impl ::dafny_runtime::DafnyPrint
    for SimpleStringConfig {
    fn fmt_print(&self, _formatter: &mut ::std::fmt::Formatter, _in_seq: bool) -> std::fmt::Result {
      match self {
        SimpleStringConfig::SimpleStringConfig { } => {
          write!(_formatter, "r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes.SimpleStringConfig.SimpleStringConfig")?;
          Ok(())
        },
      }
    }
  }

  impl Eq
    for SimpleStringConfig {}

  impl ::std::hash::Hash
    for SimpleStringConfig {
    fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
      match self {
        SimpleStringConfig::SimpleStringConfig { } => {
          
        },
      }
    }
  }

  impl ::std::default::Default
    for SimpleStringConfig {
    fn default() -> SimpleStringConfig {
      SimpleStringConfig::SimpleStringConfig {}
    }
  }

  impl ::std::convert::AsRef<SimpleStringConfig>
    for &SimpleStringConfig {
    fn as_ref(&self) -> Self {
      self
    }
  }

  pub struct ISimpleTypesStringClientCallHistory {}

  impl ISimpleTypesStringClientCallHistory {
    pub fn new() -> Self {
      ISimpleTypesStringClientCallHistory {}
    }
    pub fn _allocated() -> *mut Self {
      ::dafny_runtime::allocate::<Self>()
    }
    pub fn _allocated_rcmut() -> ::dafny_runtime::Object<Self> {
      ::dafny_runtime::allocate_rcmut::<Self>()
    }
  }

  impl ::std::default::Default
    for ISimpleTypesStringClientCallHistory {
    fn default() -> Self {
      ISimpleTypesStringClientCallHistory::new()
    }
  }

  impl ::dafny_runtime::DafnyPrint
    for ISimpleTypesStringClientCallHistory {
    fn fmt_print(&self, _formatter: &mut ::std::fmt::Formatter, _in_seq: bool) -> std::fmt::Result {
      write!(_formatter, "r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes.ISimpleTypesStringClientCallHistory")
    }
  }

  impl ::std::cmp::PartialEq
    for ISimpleTypesStringClientCallHistory {
    fn eq(&self, other: &Self) -> bool {
      ::std::ptr::eq(self, other)
    }
  }

  pub trait ISimpleTypesStringClient {
    fn GetString(&mut self, input: &::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringInput>) -> ::std::rc::Rc<super::r#_Wrappers_Compile::Result<::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>>;
    fn GetStringSingleValue(&mut self, input: &::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringInput>) -> ::std::rc::Rc<super::r#_Wrappers_Compile::Result<::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>>;
    fn GetStringUTF8(&mut self, input: &::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringInput>) -> ::std::rc::Rc<super::r#_Wrappers_Compile::Result<::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>>;
  }
 
  /*
  datatype Error =
  | CollectionOfErrors(list: seq<Error>, nameonly message: string)
  | Opaque(obj: object)
  */
  #[derive(PartialEq, Clone)]
  pub enum Error {
    CollectionOfErrors {
      list: ::dafny_runtime::Sequence<::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>,
      message: ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
    },
    Opaque {
      obj: ::dafny_runtime::Object<dyn ::std::any::Any>
    }
  }

  impl Error {
    pub fn list(&self) -> &::dafny_runtime::Sequence<::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>> {
      match self {
        Error::CollectionOfErrors { list, message, } => list,
        Error::Opaque { obj, } => panic!("field does not exist on this variant"),
      }
    }
    pub fn message(&self) -> &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16> {
      match self {
        Error::CollectionOfErrors { list, message, } => message,
        Error::Opaque { obj, } => panic!("field does not exist on this variant"),
      }
    }
    pub fn obj(&self) -> &::dafny_runtime::Object<dyn ::std::any::Any> {
      match self {
        Error::CollectionOfErrors { list, message, } => panic!("field does not exist on this variant"),
        Error::Opaque { obj, } => obj,
      }
    }
  }

  impl ::std::fmt::Debug
    for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::fmt::Result {
      ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
    }
  }

  impl ::dafny_runtime::DafnyPrint
    for Error {
    fn fmt_print(&self, _formatter: &mut ::std::fmt::Formatter, _in_seq: bool) -> std::fmt::Result {
      match self {
        Error::CollectionOfErrors { list, message, } => {
          write!(_formatter, "r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes.Error.CollectionOfErrors(")?;
          ::dafny_runtime::DafnyPrint::fmt_print(list, _formatter, false)?;
          write!(_formatter, ", ")?;
          ::dafny_runtime::DafnyPrint::fmt_print(message, _formatter, false)?;
          write!(_formatter, ")")?;
          Ok(())
        },
        Error::Opaque { obj, } => {
          write!(_formatter, "r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes.Error.Opaque(")?;
          ::dafny_runtime::DafnyPrint::fmt_print(obj, _formatter, false)?;
          write!(_formatter, ")")?;
          Ok(())
        },
      }
    }
  }

  impl Eq
    for Error {}

  impl ::std::hash::Hash
    for Error {
    fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
      match self {
        Error::CollectionOfErrors { list, message, } => {
          list.hash(_state);
          message.hash(_state)
        },
        Error::Opaque { obj, } => {
          obj.hash(_state)
        },
      }
    }
  }

  impl ::std::default::Default
    for Error {
    fn default() -> Error {
      Error::CollectionOfErrors {
        list: ::std::default::Default::default(),
        message: ::std::default::Default::default()
      }
    }
  }

  impl ::std::convert::AsRef<Error>
    for &Error {
    fn as_ref(&self) -> Self {
      self
    }
  }

  pub type OpaqueError = ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>;
}
pub mod r#_SimpleStringImpl_Compile {
  pub struct _default {}

  impl _default {
    pub fn new() -> Self {
      _default {}
    }
    pub fn _allocated() -> *mut Self {
      ::dafny_runtime::allocate::<Self>()
    }
    pub fn _allocated_rcmut() -> ::dafny_runtime::Object<Self> {
      ::dafny_runtime::allocate_rcmut::<Self>()
    }
    pub fn GetString(config: &::std::rc::Rc<super::r#_SimpleStringImpl_Compile::Config>, input: &::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringInput>) -> ::std::rc::Rc<super::r#_Wrappers_Compile::Result<::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>> {
      let mut output = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<super::r#_Wrappers_Compile::Result<::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>>>::new();
      let mut res: ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput> = ::std::rc::Rc::new(super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput::GetStringOutput {
            value: input.value().clone()
          });
      res = ::std::rc::Rc::new(super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput::GetStringOutput {
            value: input.value().clone()
          });
      output = ::dafny_runtime::MaybePlacebo::from(::std::rc::Rc::new(super::r#_Wrappers_Compile::Result::<::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>::Success {
              value: res.clone()
            }));
      return output.read();
      return output.read();
    }
    pub fn GetStringSingleValue(config: &::std::rc::Rc<super::r#_SimpleStringImpl_Compile::Config>, input: &::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringInput>) -> ::std::rc::Rc<super::r#_Wrappers_Compile::Result<::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>> {
      let mut output = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<super::r#_Wrappers_Compile::Result<::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>>>::new();
      if !matches!(input.value().as_ref(), super::r#_Wrappers_Compile::Option::Some{ .. }) {
        panic!("Halt")
      };
      if !(input.value().value().clone() == ::dafny_runtime::string_utf16_of("TEST_SIMPLE_STRING_SINGLE_VALUE")) {
        panic!("Halt")
      };
      let mut res: ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput> = ::std::rc::Rc::new(super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput::GetStringOutput {
            value: input.value().clone()
          });
      res = ::std::rc::Rc::new(super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput::GetStringOutput {
            value: input.value().clone()
          });
      output = ::dafny_runtime::MaybePlacebo::from(::std::rc::Rc::new(super::r#_Wrappers_Compile::Result::<::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>::Success {
              value: res.clone()
            }));
      return output.read();
      return output.read();
    }
    pub fn GetStringUTF8(config: &::std::rc::Rc<super::r#_SimpleStringImpl_Compile::Config>, input: &::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringInput>) -> ::std::rc::Rc<super::r#_Wrappers_Compile::Result<::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>> {
      let mut output = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<super::r#_Wrappers_Compile::Result<::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>>>::new();
      if !matches!(input.value().as_ref(), super::r#_Wrappers_Compile::Option::Some{ .. }) {
        panic!("Halt")
      };
      let mut res: ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput> = ::std::rc::Rc::new(super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput::GetStringOutput {
            value: input.value().clone()
          });
      res = ::std::rc::Rc::new(super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput::GetStringOutput {
            value: input.value().clone()
          });
      output = ::dafny_runtime::MaybePlacebo::from(::std::rc::Rc::new(super::r#_Wrappers_Compile::Result::<::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>::Success {
              value: res.clone()
            }));
      return output.read();
      return output.read();
    }
  }

  impl ::std::default::Default
    for _default {
    fn default() -> Self {
      _default::new()
    }
  }

  impl ::dafny_runtime::DafnyPrint
    for _default {
    fn fmt_print(&self, _formatter: &mut ::std::fmt::Formatter, _in_seq: bool) -> std::fmt::Result {
      write!(_formatter, "r#_SimpleStringImpl_Compile._default")
    }
  }

  impl ::std::cmp::PartialEq
    for _default {
    fn eq(&self, other: &Self) -> bool {
      ::std::ptr::eq(self, other)
    }
  }

  #[derive(PartialEq, Clone)]
  pub enum Config {
    Config {}
  }

  impl Config {}

  impl ::std::fmt::Debug
    for Config {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::fmt::Result {
      ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
    }
  }

  impl ::dafny_runtime::DafnyPrint
    for Config {
    fn fmt_print(&self, _formatter: &mut ::std::fmt::Formatter, _in_seq: bool) -> std::fmt::Result {
      match self {
        Config::Config { } => {
          write!(_formatter, "r#_SimpleStringImpl_Compile.Config.Config")?;
          Ok(())
        },
      }
    }
  }

  impl Eq
    for Config {}

  impl ::std::hash::Hash
    for Config {
    fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
      match self {
        Config::Config { } => {
          
        },
      }
    }
  }

  impl ::std::default::Default
    for Config {
    fn default() -> Config {
      Config::Config {}
    }
  }

  impl ::std::convert::AsRef<Config>
    for &Config {
    fn as_ref(&self) -> Self {
      self
    }
  }
}
// SimpleString
pub mod r#_simple_dtypes_dsmithystring_dinternaldafny {
  pub struct _default {}

  impl _default {
    pub fn new() -> Self {
      _default {}
    }
    pub fn _allocated() -> *mut Self {
      ::dafny_runtime::allocate::<Self>()
    }
    pub fn _allocated_rcmut() -> ::dafny_runtime::Object<Self> {
      ::dafny_runtime::allocate_rcmut::<Self>()
    }
    pub fn DefaultSimpleStringConfig() -> ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::SimpleStringConfig> {
      ::std::rc::Rc::new(super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::SimpleStringConfig::SimpleStringConfig {})
    }
    /*method SimpleString(config: SimpleStringConfig)
    returns (res: Result<ISimpleTypesStringClient, Error>) {
      var client := new SimpleStringClient(Operations.Config);
      return Success(client);
    } */
    pub fn SimpleString(config: &::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::SimpleStringConfig>) -> ::std::rc::Rc<super::r#_Wrappers_Compile::Result<::dafny_runtime::Object<dyn super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::ISimpleTypesStringClient>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>> {
      let mut res = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<super::r#_Wrappers_Compile::Result<_, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>>>::new();
      let mut client: ::dafny_runtime::Object<super::r#_simple_dtypes_dsmithystring_dinternaldafny::SimpleStringClient> = ::dafny_runtime::Object(None);
      let mut _nw1: ::dafny_runtime::Object<super::r#_simple_dtypes_dsmithystring_dinternaldafny::SimpleStringClient> = super::r#_simple_dtypes_dsmithystring_dinternaldafny::SimpleStringClient::_allocated_rcmut();
      super::r#_simple_dtypes_dsmithystring_dinternaldafny::SimpleStringClient::_ctor(_nw1.clone(), &::std::rc::Rc::new(super::r#_SimpleStringImpl_Compile::Config::Config {}));
      client = _nw1;
      let v: ::dafny_runtime::Object<dyn super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::ISimpleTypesStringClient> = <::dafny_runtime::Object<SimpleStringClient> as ::dafny_runtime::UpcastTo<::dafny_runtime::Object<dyn super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::ISimpleTypesStringClient>>>::upcast_to(client.clone());
      // build a success
      res = ::dafny_runtime::MaybePlacebo::from(::std::rc::Rc::new(super::r#_Wrappers_Compile::Result::<::dafny_runtime::Object<dyn super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::ISimpleTypesStringClient>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>::Success{
        value: v
      }));
      return res.read();
      return res.read();
    }
    pub fn CreateSuccessOfClient(client: &::dafny_runtime::Object<dyn super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::ISimpleTypesStringClient>) -> ::std::rc::Rc<super::r#_Wrappers_Compile::Result<::dafny_runtime::Object<dyn super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::ISimpleTypesStringClient>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>> {
      ::std::rc::Rc::new(super::r#_Wrappers_Compile::Result::<::dafny_runtime::Object<dyn super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::ISimpleTypesStringClient>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>::Success {
          value: client.clone()
        })
    }
    pub fn CreateFailureOfError(error: &::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>) -> ::std::rc::Rc<super::r#_Wrappers_Compile::Result<::dafny_runtime::Object<dyn super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::ISimpleTypesStringClient>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>> {
      ::std::rc::Rc::new(super::r#_Wrappers_Compile::Result::<::dafny_runtime::Object<dyn super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::ISimpleTypesStringClient>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>::Failure {
          error: error.clone()
        })
    }
  }

  impl ::std::default::Default
    for _default {
    fn default() -> Self {
      _default::new()
    }
  }

  impl ::dafny_runtime::DafnyPrint
    for _default {
    fn fmt_print(&self, _formatter: &mut ::std::fmt::Formatter, _in_seq: bool) -> std::fmt::Result {
      write!(_formatter, "r#_simple_dtypes_dsmithystring_dinternaldafny._default")
    }
  }

  impl ::std::cmp::PartialEq
    for _default {
    fn eq(&self, other: &Self) -> bool {
      ::std::ptr::eq(self, other)
    }
  }

  pub struct SimpleStringClient {
    pub r#__i_config: ::std::rc::Rc<super::r#_SimpleStringImpl_Compile::Config>
  }

  impl SimpleStringClient {
    pub fn new() -> Self {
      SimpleStringClient {
        r#__i_config: <::std::rc::Rc<super::r#_SimpleStringImpl_Compile::Config> as std::default::Default>::default()
      }
    }
    pub fn _allocated_rcmut() -> ::dafny_runtime::Object<Self> {
      ::dafny_runtime::allocate_rcmut::<Self>()
    }
    pub fn _allocated() -> *mut Self {
      ::dafny_runtime::allocate::<Self>()
    }
    fn _ctor(this: ::dafny_runtime::Object<SimpleStringClient>, config: &::std::rc::Rc<super::r#_SimpleStringImpl_Compile::Config>) -> () {
      let mut _set__i_config: bool = false;
      ::dafny_runtime::update_field_uninit_rcmut!(this, r#__i_config, _set__i_config, config.clone());
      return ();
    }
    pub fn config(&self) -> ::std::rc::Rc<super::r#_SimpleStringImpl_Compile::Config> {
      self.r#__i_config.clone()
    }
  }

  impl super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::ISimpleTypesStringClient
    for super::r#_simple_dtypes_dsmithystring_dinternaldafny::SimpleStringClient {
    fn GetString(&mut self, input: &::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringInput>) -> ::std::rc::Rc<super::r#_Wrappers_Compile::Result<::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>> {
      let mut output = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<super::r#_Wrappers_Compile::Result<::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>>>::new();
      let mut _out0 = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<super::r#_Wrappers_Compile::Result<::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>>>::new();
      _out0 = ::dafny_runtime::MaybePlacebo::from(super::r#_SimpleStringImpl_Compile::_default::GetString(&self.config(), input));
      output = ::dafny_runtime::MaybePlacebo::from(_out0.read());
      return output.read();
    }
    fn GetStringSingleValue(&mut self, input: &::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringInput>) -> ::std::rc::Rc<super::r#_Wrappers_Compile::Result<::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>> {
      let mut output = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<super::r#_Wrappers_Compile::Result<::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>>>::new();
      let mut _out1 = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<super::r#_Wrappers_Compile::Result<::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>>>::new();
      _out1 = ::dafny_runtime::MaybePlacebo::from(super::r#_SimpleStringImpl_Compile::_default::GetStringSingleValue(&self.config(), input));
      output = ::dafny_runtime::MaybePlacebo::from(_out1.read());
      return output.read();
    }
    fn GetStringUTF8(&mut self, input: &::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringInput>) -> ::std::rc::Rc<super::r#_Wrappers_Compile::Result<::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>> {
      let mut output = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<super::r#_Wrappers_Compile::Result<::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>>>::new();
      let mut _out2 = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<super::r#_Wrappers_Compile::Result<::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::GetStringOutput>, ::std::rc::Rc<super::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::Error>>>>::new();
      _out2 = ::dafny_runtime::MaybePlacebo::from(super::r#_SimpleStringImpl_Compile::_default::GetStringUTF8(&self.config(), input));
      output = ::dafny_runtime::MaybePlacebo::from(_out2.read());
      return output.read();
    }
  }

  impl ::std::default::Default
    for SimpleStringClient {
    fn default() -> Self {
      SimpleStringClient::new()
    }
  }

  impl ::dafny_runtime::DafnyPrint
    for SimpleStringClient {
    fn fmt_print(&self, _formatter: &mut ::std::fmt::Formatter, _in_seq: bool) -> std::fmt::Result {
      write!(_formatter, "r#_simple_dtypes_dsmithystring_dinternaldafny.SimpleStringClient")
    }
  }

  impl ::std::cmp::PartialEq
    for SimpleStringClient {
    fn eq(&self, other: &Self) -> bool {
      ::std::ptr::eq(self, other)
    }
  }
}
pub mod r#_StandardLibraryInterop_Compile {
  pub struct WrappersInterop {}

  impl WrappersInterop {
    pub fn new() -> Self {
      WrappersInterop {}
    }
    pub fn _allocated_rcmut() -> ::dafny_runtime::Object<Self> {      
      ::dafny_runtime::allocate_rcmut::<Self>()         
    }
    pub fn _allocated() -> *mut Self {
      ::dafny_runtime::allocate::<Self>()
    }
    pub fn CreateStringSome(s: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>) -> ::std::rc::Rc<super::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>> {
      ::std::rc::Rc::new(super::r#_Wrappers_Compile::Option::<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>::Some {
          value: s.clone()
        })
    }
    pub fn CreateStringNone() -> ::std::rc::Rc<super::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>> {
      ::std::rc::Rc::new(super::r#_Wrappers_Compile::Option::<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>::None {})
    }
    pub fn CreateBooleanSome(b: bool) -> ::std::rc::Rc<super::r#_Wrappers_Compile::Option<bool>> {
      ::std::rc::Rc::new(super::r#_Wrappers_Compile::Option::<bool>::Some {
          value: b
        })
    }
    pub fn CreateBooleanNone() -> ::std::rc::Rc<super::r#_Wrappers_Compile::Option<bool>> {
      ::std::rc::Rc::new(super::r#_Wrappers_Compile::Option::<bool>::None {})
    }
  }

  impl ::std::default::Default
    for WrappersInterop {
    fn default() -> Self {
      WrappersInterop::new()
    }
  }

  impl ::dafny_runtime::DafnyPrint
    for WrappersInterop {
    fn fmt_print(&self, _formatter: &mut ::std::fmt::Formatter, _in_seq: bool) -> std::fmt::Result {
      write!(_formatter, "r#_StandardLibraryInterop_Compile.WrappersInterop")
    }
  }

  impl ::std::cmp::PartialEq
    for WrappersInterop {
    fn eq(&self, other: &Self) -> bool {
      ::std::ptr::eq(self, other)
    }
  }
}
pub mod _module {
  
}