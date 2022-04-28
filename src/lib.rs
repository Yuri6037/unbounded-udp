// Copyright (c) 2022, Yuri6037
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
// * Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation
// and/or other materials provided with the distribution.
// * Neither the name of Yuri6037 nor the names of its contributors
// may be used to endorse or promote products derived from this software
// without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::net::UdpSocket;
use std::io::Result;
use std::io::Error;

pub enum Domain {
    IpV4,
    Ipv6
}

mod sealing {
    pub trait Unbounded {}

    impl Unbounded for std::net::UdpSocket {}
}

pub trait Unbounded : sealing::Unbounded where Self: Sized {
    fn unbounded(domain: Domain) -> Result<Self>;
}

impl Unbounded for UdpSocket {
    fn unbounded(domain: Domain) -> Result<Self> {
        #[cfg(unix)]
        {
            use std::os::unix::io::FromRawFd;
            let domain = match domain {
                Domain::IpV4 => libc::AF_INET,
                Domain::Ipv6 => libc::AF_INET6
            };
            let fd = unsafe { libc::socket(domain, libc::SOCK_DGRAM, libc::IPPROTO_UDP) };
            if fd == -1 {
                return Err(Error::last_os_error());
            }
            let socket = unsafe { UdpSocket::from_raw_fd(fd) };
            Ok(socket)
        }
        #[cfg(windows)]
        {
            use std::os::windows::io::FromRawSocket;
            use std::os::windows::io::RawSocket;
            use windows_sys::Win32::Networking::WinSock::SOCK_DGRAM;
            use windows_sys::Win32::Networking::WinSock::IPPROTO_UDP;
            use windows_sys::Win32::Networking::WinSock::AF_INET;
            use windows_sys::Win32::Networking::WinSock::AF_INET6;
            use windows_sys::Win32::Networking::WinSock::socket;
            use windows_sys::Win32::Networking::WinSock::WSAGetLastError;
            use windows_sys::Win32::Networking::WinSock::INVALID_SOCKET;
            let domain = match domain {
                Domain::IpV4 => AF_INET,
                Domain::Ipv6 => AF_INET6
            };
            let sock = unsafe { socket(domain as i32, SOCK_DGRAM as i32, IPPROTO_UDP) };
            if sock == INVALID_SOCKET {
                return Err(Error::from_raw_os_error(unsafe { WSAGetLastError() }));
            }
            let socket = unsafe { UdpSocket::from_raw_socket(sock as RawSocket) };
            Ok(socket)
        }
    }
}
