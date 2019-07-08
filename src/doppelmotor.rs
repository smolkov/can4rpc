use futures::prelude::*;
use serde_derive::{Deserialize,Serialize};
use std::os::raw::*;
use tide::{error::ResultExt, Context, EndpointResult};

/// Motor oder Doppelmotor
/// 6X00:1 ParameterName=Stepper=0, Ruhrer =1 ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=1 PDOMapping=0
/// 6X00:2 ParameterName=Endschalter, geschlossen=1 ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=0 PDOMapping=0
/// 6X00:3 ParameterName=Endschalter invertieren ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
/// 6X01:1 ParameterName=Command / Status 0/1 ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
/// 6X01:2 ParameterName=Command  go to Pos ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=0 PDOMapping=0
/// 6X01:3 ParameterName=Position Old ObjectType=0x7 DataType=0x0006 AccessType=ro DefaultValue=0 PDOMapping=0
/// 6X01:4 - ParameterName=Max Position ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=1800 PDOMapping=0
/// 6X01:5 - ParameterName=Fahrparameter ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=1 PDOMapping=0
/// 6X01:6 - ParameterName=Fahrparameter ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=1 PDOMapping=0
///  Typed:
///     0x001 - bool
///     0x002 - u8
///     0x003 - u16
///     0x004 - i16
///     0x006 - i16
///  Addressen:
///  `6100` - ParameterName=Stepper 1 / Ruhrer 1 ObjectType=0x8 SubNumber=4
///       0 - ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=3 PDOMapping=0
///       1 - ParameterName=Stepper=0, Ruhrer =1 ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=1 PDOMapping=0
///       2 - ParameterName=Endschalter, geschlossen=1 ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=0 PDOMapping=0
///       3 - ParameterName=Endschalter invertieren ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
///       4 - ParameterName=Diagnose TCM249 ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
///  `6101` - ParameterName=Stepper 1 - Position ObjectType=0x8 SubNumber=5
///       0 - ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=5 PDOMapping=0
///       1 - ParameterName=Command / Status 0/1 ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
///       2 - ParameterName=Command  go to Pos ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=0 PDOMapping=0
///       3 - ParameterName=Position Old ObjectType=0x7 DataType=0x0006 AccessType=ro DefaultValue=0 PDOMapping=0
///       4 - ParameterName=Max Position ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=1800 PDOMapping=0
///       5 - ParameterName=Fahrparameter ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=1 PDOMapping=0
///       6 - ParameterName=Stall guard flag ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=1 PDOMapping=0
///  `6102` - ParameterName=Ruhrer 1 ObjectType=0x8 SubNumber=3
///       0 - ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=2 PDOMapping=0
///       1 - ParameterName=Aus=0, Ein=1 ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=1 PDOMapping=0
///       2 - ParameterName=Delay ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=-126 PDOMapping=0
///  `6103` - ParameterName=Strom 1 ObjectType=0x8 SubNumber=5
///      `0` - ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=4 PDOMapping=0
///      `1` - ParameterName=Stromsollwert / mA ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=600 PDOMapping=0
///      `2` - ParameterName=Stromwert Digital ObjectType=0x7 DataType=0x0006 AccessType=ro DefaultValue=204 PDOMapping=0
///      `3` - ParameterName=Haltestromsollwert / mA ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=150 PDOMapping=0
///      `4` - ParameterName=Haltestromwert Digital ObjectType=0x7 DataType=0x0006 AccessType=ro DefaultValue=51 PDOMapping=0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stepper {
    node: u32,
/// Mode Stepper,Stirrer
    pub index: u16,
    pub state: u8,
    pub endshalter: u8,
    pub invert_endshalter: u8,
    pub command: u8,
    pub position: i16,
    pub position_old: i16,
    pub max_pos: i16,
    pub parameter: u8,
    pub stall_guard: u8,
    pub stirrer_run: u8,
    pub delay:i8,
    pub stromsolwert: i16,
    pub stromwert_digital:i16,
    pub haltestromsollwert: i16,
    pub haltestromsollwert_digital: i16,
}

impl Default for Stepper {
    fn default() -> Self {
        Self {
            node: 0x12,
            index:6100,
            state: 1,
            endshalter: 0,
            invert_endshalter:0,
            command: 0,
            position: 0,
            position_old: 0,
            max_pos: 1800,
            parameter: 1,
            stall_guard: 1,
            stirrer_run: 0,
            delay:-126,
            stromsolwert: 600,
            stromwert_digital:204,
            haltestromsollwert: 150,
            haltestromsollwert_digital:51,
        }
    }
}


impl Stepper {
    fn new(node: u32, index: u16) -> Self {
        Self {
            node: node,
            index:index,
            state: 1,
            endshalter: 0,
            invert_endshalter:0,
            command: 0,
            position: 0,
            position_old: 0,
            max_pos: 1800,
            parameter: 1,
            stall_guard: 1,
            stirrer_run: 0,
            delay:-126,
            stromsolwert: 600,
            stromwert_digital:204,
            haltestromsollwert: 150,
            haltestromsollwert_digital:51,
        }
    }
}
/// Pump controlling.
/// * 6110 ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=8 PDOMapping=0
///     - 1 ParameterName=0=Stop, 1=Start 2=Wait ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
///     - 2 ParameterName=0=Normal, 1=Spulen,2=Intervall ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
///     - 3 ParameterName=Drehrichtung rechts / links ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
///     - 4 ParameterName=Speed soll ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
///     - 5 ParameterName=Interwall Pos-Impulse ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=0 PDOMapping=0
///     - 6 ParameterName=Interwall Time / sec ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=0 PDOMapping=0
///     - 7 ParameterName=Position ObjectType=0x7 DataType=0x0006 AccessType=ro DefaultValue=0 PDOMapping=0
///     - 8 ParameterName=Delay PW ObjectType=0x7 DataType=0x0006 AccessType=ro DefaultValue=0 PDOMapping=0
/// * 6111  ParameterName=Konstanten 1 ObjectType=0x8 SubNumber=4
///     - 0  ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=3 PDOMapping=0
///     - 1  ParameterName=K-p ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=1 PDOMapping=0
///     - 2  ParameterName=K-d ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=1 PDOMapping=0
///     - 3  ParameterName=K-i ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=1 PDOMapping=0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pump {
    /// Node id
    node: u32,
    /// Index 6110=Part1 6210=Part2
    pub index: u16,
    /// Status 0=Stop, 1=Start 2=Wait
    pub status: u8,
    /// Runstate 0=Normal, 1=Spulen,2=Intervall
    pub runstate: u8,
    pub drehrichtung: u8,
    pub speed:u8,
    pub interval_impuls: i16,
    pub interval_time:i16,
    pub position:i16,
    pub delay:i16,
    pub cons_kp:i16,
    pub cons_kd:i16,
    pub cons_ki:i16,
}
impl Default for Pump {
    fn default() -> Self{
        Self {
            node: 0x12,
            index:6110,
            status: 0,
            runstate:0,
            drehrichtung: 0,
            speed:0,
            interval_impuls:0,
            interval_time:0,
            position:0,
            delay:0,
            cons_kp:0,
            cons_kd:0,
            cons_ki:0,

        }
    }
}


impl Pump {
    fn new(node:u32,index:u16) ->Self {
        Self {
            node: node,
            index:index,
            status: 0,
            runstate:0,
            drehrichtung: 0,
            speed:0,
            interval_impuls:0,
            interval_time:0,
            position:0,
            delay:0,
            cons_kp:0,
            cons_kd:0,
            cons_ki:0,

        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Uart {
    node : u32,
    pub index: u16,
    pub data : Vec<u8>,
    pub baut : u8,
}

impl Default for Uart {
    fn default() -> Self{
        Self {
            node: 0x12,
            index:6000,
            data: Vec::new(),
            baut: 9
        }
    }
}

impl Uart {
    fn new(node:u32,index:u16) ->Self {
        Self {
            node: node,
            index:index,
            data: Vec::new(),
            baut: 9
        }
    }
}
// [6100sub1]
// ParameterName=Stepper=0, Ruhrer =1
// ObjectType=0x7
// DataType=0x0002
// AccessType=rw
// DefaultValue=1
// PDOMapping=0
// [6100sub2]
// ParameterName=Endschalter, geschlossen=1
// ObjectType=0x7
// DataType=0x0002
// AccessType=ro
// DefaultValue=0
// PDOMapping=0
pub struct DoppelmotorNode{
    /// Index 6100 ,6101
    pub node: u32,
    pub motor1: Stepper,
    pub motor2: Stepper,
    pub pump1: Pump,
    pub pump2: Pump,
    pub uart1: Uart,
    pub uart2: Uart,
}


impl Default for DoppelmotorNode {
    fn default() -> Self{
        let node = 0x12 as u32;
        let motor1 = Stepper::new(0x12,6100);
        let motor2 = Stepper::new(0x12,6200);
        let pump1  = Pump::new(0x12,6110);
        let pump2  = Pump::new(0x12,6210);
        let uart1  = Uart::new(0x12,6000);
        let uart2  = Uart::new(0x12,6010);
        Self {
            node,motor1,motor2,pump1,pump2,uart1,uart2
        }
    }
}


impl DoppelmotorNode {
    pub fn new(node : u32) -> DoppelmotorNode {
        DoppelmotorNode {
           node: node,
           motor1: Stepper::new(node,6100),
           motor2: Stepper::new(node,6200),
           pump1:  Pump::new(node,6110),
           pump2:  Pump::new(node,6210),
           uart1:  Uart::new(node,6000),
           uart2:  Uart::new(node,6010),
        }
    }
}

extern {
    fn doppelmotor_get_uart01(node:c_int) -> *mut c_uchar;
    fn doppelmotor_set_baut01(node:c_int,val:c_uint)->c_int;
    fn doppelmotor_set_uart01(node:c_int,data: *mut c_uchar) -> c_int;
    fn doppelmotor_get_uart02(node:c_int) -> *mut c_uchar;
    fn doppelmotor_set_uart02(node:c_int,data: *mut c_uchar) -> c_int;
    fn doppelmotor_set_baut02(node:c_int,val:c_uint)->c_uint;
}


pub async fn get_uart01(cx: Context<()>) -> EndpointResult<Vec<u8>> {
    let node:i32 = cx.param("id").client_err()?;
    let uart:Vec<u8> = Vec::new();
    unsafe{
      let t = doppelmotor_get_uart01(node);
    }
    Ok(uart.clone())
}

pub async fn set_uart01(mut cx: Context<()>)  -> EndpointResult<()>  {
    let node:i32 = cx.param("id").client_err()?;
    let mut value:Vec<u8> = cx.body_json().await.client_err()?;
    unsafe{
        doppelmotor_set_uart01(node,value.as_mut_ptr());
    }
    Ok(())
}

pub async fn setup_uart01(mut cx: Context<()>) -> EndpointResult<()> {
    let node:i32 = cx.param("id").client_err()?;
    let baut:u32 = cx.body_json().await.client_err()?;
    unsafe{
      doppelmotor_set_baut01(node,baut);
    }
    Ok(())
}

pub async fn get_uart02(cx: Context<()>) -> EndpointResult<Vec<u8>> {
    let node:i32 = cx.param("id").client_err()?;
    let uart:Vec<u8> = Vec::new();
    unsafe{
      let t = doppelmotor_get_uart02(node);
    }
    Ok(uart.clone())
}

pub async fn set_uart02(mut cx: Context<()>)  -> EndpointResult<()>  {
    let node:i32 = cx.param("id").client_err()?;
    let mut value:Vec<u8> = cx.body_json().await.client_err()?;
    unsafe{
        doppelmotor_set_uart02(node,value.as_mut_ptr());
    }
    Ok(())
}

pub async fn setup_uart02(mut cx: Context<()>) -> EndpointResult<()> {
    let node:i32 = cx.param("id").client_err()?;
    let baut:u32 = cx.body_json().await.client_err()?;
    unsafe{
      doppelmotor_set_baut02(node,baut);
    }
    Ok(())
}