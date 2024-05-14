use std::{
    ffi::c_void,
    fmt,
    mem::{size_of, transmute},
    time::Instant,
    thread,
};

type HModule = *const c_void;
type FarProc = *const c_void;

extern "stdcall" {
    fn LoadLibraryA(name: *const u8) -> HModule;
    fn GetProcAddress(module: HModule, name: *const u8) -> FarProc;
}

#[repr(transparent)]
#[derive(Copy, Clone)]
struct IPAddr([u8; 4]);

impl fmt::Debug for IPAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let [a, b, c, d] = self.0;
        write!(f, "{}.{}.{}.{}", a, b, c, d)
    }
}

#[repr(C)]
#[derive(Debug)]
struct IpOptionInformation {
    ttl: u8,
    tos: u8,
    flags: u8,
    options_size: u8,
    options_data: u32,
}

type Handle = *const c_void;

#[repr(C)]
#[derive(Debug)]
struct IcmpEchoReply {
    address: IPAddr,
    status: u32,
    rtt: u32, 
    data_size: u16,
    reserved: u16,
    data: *const u8,
    options: IpOptionInformation,
}

type IcmpSendEcho = extern "stdcall" fn(
    handle: Handle,
    dest: IPAddr,
    request_data: *const u8,
    request_size: u16,
    request_options: Option<&IpOptionInformation>,
    reply_buffer: *mut u8,
    reply_size: u32,
    timeout: u32,
) -> u32;

type IcmpCreateFile = extern "stdcall" fn() -> Handle;

fn parse_ipv4(ip_str: &str) -> Option<IPAddr> {
    let parts: Vec<u8> = ip_str
        .split('.')
        .filter_map(|s| s.parse::<u8>().ok())
        .collect();
    if parts.len() == 4 {
        Some(IPAddr([parts[0], parts[1], parts[2], parts[3]]))
    } else {
        None
    }
}

fn draw_box(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    let top = format!("┌{:─^width$}┐", "", width = width);
    let bottom = format!("└{:─^width$}┘", "", width = width);

    let middle: Vec<String> = lines
        .into_iter()
        .map(|line| format!("│{: ^width$}│", line, width = width))
        .collect();

    format!("{}\n{}\n{}", top, middle.join("\n"), bottom)
}

pub fn main(destination: Option<&str>, print_all: bool) {
    #[allow(non_snake_case)]
    
    unsafe {
        let h = LoadLibraryA("IPHLPAPI.dll\0".as_ptr());
        let IcmpCreateFile: IcmpCreateFile =
            transmute(GetProcAddress(h, "IcmpCreateFile\0".as_ptr()));
        let IcmpSendEcho: IcmpSendEcho = transmute(GetProcAddress(h, "IcmpSendEcho\0".as_ptr()));

        let handle = IcmpCreateFile();

        let data = "Green Fn!";
        let ip_opts = IpOptionInformation {
            ttl: 128,
            tos: 0,
            flags: 0,
            options_data: 0,
            options_size: 0,
        };

        let reply_size = size_of::<IcmpEchoReply>();
        let reply_buf_size = reply_size + 8 + data.len();
        let mut reply_buf = vec![0u8; reply_buf_size];

        let num_pings = 5;  
        let destination_ip = match destination.and_then(parse_ipv4) {
            Some(ip) => ip,
            None => IPAddr([1,1,1,1]),
        };  // Target IP address

        
        println!("\n");
        for i in 0..num_pings {
            let start = Instant::now();
            let ret = IcmpSendEcho(
                handle,
                destination_ip,
                data.as_ptr(),
                data.len() as u16,
                Some(&ip_opts),
                reply_buf.as_mut_ptr(),
                reply_buf_size as u32,
                4000, // Timeout in milliseconds
            );

            let elapsed = start.elapsed();
            
            #[allow(unused_assignments)]
            let mut response = String::new();
            if ret == 0 {
                response = format!(
                    "Reply from {:#?}: Request timed out.",
                    destination_ip
                );
            } else {
                let reply = &*(reply_buf.as_ptr() as *const IcmpEchoReply);
                let rtt = elapsed.as_millis();
                response = format!(
                    "PONG from {:#?}: bytes={} time={}ms TTL={}",
                    reply.address, reply.data_size, rtt, reply.options.ttl
                );

                if print_all {
                    response.push_str(&format!(
                        "\n\nPing statistics for {:#?}: \n{:#?}",
                        destination_ip, reply.options
                    ));
                }
                else {
                    response.push_str(&format!(
                        "\nAdditional info: status={}, flags={}, tos={}",
                        reply.status, reply.options.flags, reply.options.tos
                    ));
                }

                
            }

            println!("{}", draw_box(&response));

            if i < num_pings - 1 {
                thread::sleep(std::time::Duration::from_secs(1));
            }
        }
        // Add any necessary ping statistics here
    }
}
