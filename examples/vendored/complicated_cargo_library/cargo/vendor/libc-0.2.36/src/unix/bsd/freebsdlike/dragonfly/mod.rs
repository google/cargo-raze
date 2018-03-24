pub type clock_t = u64;
pub type ino_t = u64;
pub type nlink_t = u32;
pub type blksize_t = i64;
pub type clockid_t = ::c_ulong;

pub type c_long = i64;
pub type c_ulong = u64;
pub type time_t = i64;
pub type suseconds_t = i64;

pub type uuid_t = ::uuid;

pub type fsblkcnt_t = u64;
pub type fsfilcnt_t = u64;

pub type sem_t = *mut sem;

pub enum sem {}

s! {

    pub struct exit_status {
        pub e_termination: u16,
        pub e_exit: u16
    }

    pub struct utmpx {
        pub ut_name: [::c_char; 32],
        pub ut_id: [::c_char; 4],

        pub ut_line: [::c_char; 32],
        pub ut_host: [::c_char; 256],

        pub ut_unused: [u8; 16],
        pub ut_session: u16,
        pub ut_type: u16,
        pub ut_pid: ::pid_t,
        ut_exit: exit_status,
        ut_ss: ::sockaddr_storage,
        pub ut_tv: ::timeval,
        pub ut_unused2: [u8; 16],
    }

    pub struct aiocb {
        pub aio_fildes: ::c_int,
        pub aio_offset: ::off_t,
        pub aio_buf: *mut ::c_void,
        pub aio_nbytes: ::size_t,
        pub aio_sigevent: sigevent,
        pub aio_lio_opcode: ::c_int,
        pub aio_reqprio: ::c_int,
        _aio_val: ::c_int,
        _aio_err: ::c_int
    }

    pub struct dirent {
        pub d_fileno: ::ino_t,
        pub d_namlen: u16,
        pub d_type: u8,
        __unused1: u8,
        __unused2: u32,
        pub d_name: [::c_char; 256],
    }

    pub struct uuid {
        pub time_low: u32,
        pub time_mid: u16,
        pub time_hi_and_version: u16,
        pub clock_seq_hi_and_reserved: u8,
        pub clock_seq_low: u8,
        pub node: [u8; 6],
    }

    pub struct mq_attr {
        pub mq_flags: ::c_long,
        pub mq_maxmsg: ::c_long,
        pub mq_msgsize: ::c_long,
        pub mq_curmsgs: ::c_long,
    }

    pub struct sigevent {
        pub sigev_notify: ::c_int,
        // The union is 8-byte in size, so it is aligned at a 8-byte offset.
        #[cfg(target_pointer_width = "64")]
        __unused1: ::c_int,
        pub sigev_signo: ::c_int,       //actually a union
        // pad the union
        #[cfg(target_pointer_width = "64")]
        __unused2: ::c_int,
        pub sigev_value: ::sigval,
        __unused3: *mut ::c_void        //actually a function pointer
    }

    pub struct statvfs {
        pub f_bsize: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_blocks: ::fsblkcnt_t,
        pub f_bfree: ::fsblkcnt_t,
        pub f_bavail: ::fsblkcnt_t,
        pub f_files: ::fsfilcnt_t,
        pub f_ffree: ::fsfilcnt_t,
        pub f_favail: ::fsfilcnt_t,
        pub f_fsid: ::c_ulong,
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
        pub f_owner: ::uid_t,
        pub f_type: ::c_uint,
        pub f_syncreads: u64,
        pub f_syncwrites: u64,
        pub f_asyncreads: u64,
        pub f_asyncwrites: u64,
        pub f_fsid_uuid: ::uuid_t,
        pub f_uid_uuid: ::uuid_t,
    }

    pub struct stat {
        pub st_ino: ::ino_t,
        pub st_nlink: ::nlink_t,
        pub st_dev: ::dev_t,
        pub st_mode: ::mode_t,
        pub st_padding1: ::uint16_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        pub st_size: ::off_t,
        pub st_blocks: ::int64_t,
        pub st_blksize: ::uint32_t,
        pub st_flags: ::uint32_t,
        pub st_gen: ::uint32_t,
        pub st_lspare: ::int32_t,
        pub st_qspare1: ::int64_t,
        pub st_qspare2: ::int64_t,
    }

    pub struct if_data {
        pub ifi_type: ::c_uchar,
        pub ifi_physical: ::c_uchar,
        pub ifi_addrlen: ::c_uchar,
        pub ifi_hdrlen: ::c_uchar,
        pub ifi_recvquota: ::c_uchar,
        pub ifi_xmitquota: ::c_uchar,
        pub ifi_mtu: ::c_ulong,
        pub ifi_metric: ::c_ulong,
        pub ifi_link_state: ::c_ulong,
        pub ifi_baudrate: u64,
        pub ifi_ipackets: ::c_ulong,
        pub ifi_ierrors: ::c_ulong,
        pub ifi_opackets: ::c_ulong,
        pub ifi_oerrors: ::c_ulong,
        pub ifi_collisions: ::c_ulong,
        pub ifi_ibytes: ::c_ulong,
        pub ifi_obytes: ::c_ulong,
        pub ifi_imcasts: ::c_ulong,
        pub ifi_omcasts: ::c_ulong,
        pub ifi_iqdrops: ::c_ulong,
        pub ifi_noproto: ::c_ulong,
        pub ifi_hwassist: ::c_ulong,
        pub ifi_oqdrops: ::c_ulong,
        pub ifi_lastchange: ::timeval,
    }

    pub struct if_msghdr {
        pub ifm_msglen: ::c_ushort,
        pub ifm_version: ::c_uchar,
        pub ifm_type: ::c_uchar,
        pub ifm_addrs: ::c_int,
        pub ifm_flags: ::c_int,
        pub ifm_index: ::c_ushort,
        pub ifm_data: if_data,
    }

    pub struct sockaddr_dl {
        pub sdl_len: ::c_uchar,
        pub sdl_family: ::c_uchar,
        pub sdl_index: ::c_ushort,
        pub sdl_type: ::c_uchar,
        pub sdl_nlen: ::c_uchar,
        pub sdl_alen: ::c_uchar,
        pub sdl_slen: ::c_uchar,
        pub sdl_data: [::c_char; 12],
        pub sdl_rcf: ::c_ushort,
        pub sdl_route: [::c_ushort; 16],
    }
}

pub const RAND_MAX: ::c_int = 0x7fff_ffff;
pub const PTHREAD_STACK_MIN: ::size_t = 16384;
pub const SIGSTKSZ: ::size_t = 40960;
pub const MADV_INVAL: ::c_int = 10;
pub const MADV_SETMAP: ::c_int = 11;
pub const O_CLOEXEC: ::c_int = 0x00020000;
pub const O_DIRECTORY: ::c_int = 0x08000000;
pub const F_GETLK: ::c_int = 7;
pub const F_SETLK: ::c_int = 8;
pub const F_SETLKW: ::c_int = 9;
pub const ELAST: ::c_int = 99;
pub const RLIMIT_POSIXLOCKS: ::c_int = 11;
pub const RLIM_NLIMITS: ::rlim_t = 12;

pub const Q_GETQUOTA: ::c_int = 0x300;
pub const Q_SETQUOTA: ::c_int = 0x400;

pub const CLOCK_REALTIME: ::clockid_t = 0;
pub const CLOCK_VIRTUAL: ::clockid_t = 1;
pub const CLOCK_PROF: ::clockid_t = 2;
pub const CLOCK_MONOTONIC: ::clockid_t = 4;
pub const CLOCK_UPTIME: ::clockid_t = 5;
pub const CLOCK_UPTIME_PRECISE: ::clockid_t = 7;
pub const CLOCK_UPTIME_FAST: ::clockid_t = 8;
pub const CLOCK_REALTIME_PRECISE: ::clockid_t = 9;
pub const CLOCK_REALTIME_FAST: ::clockid_t = 10;
pub const CLOCK_MONOTONIC_PRECISE: ::clockid_t = 11;
pub const CLOCK_MONOTONIC_FAST: ::clockid_t = 12;
pub const CLOCK_SECOND: ::clockid_t = 13;
pub const CLOCK_THREAD_CPUTIME_ID: ::clockid_t = 14;
pub const CLOCK_PROCESS_CPUTIME_ID: ::clockid_t = 15;

pub const CTL_UNSPEC: ::c_int = 0;
pub const CTL_KERN: ::c_int = 1;
pub const CTL_VM: ::c_int = 2;
pub const CTL_VFS: ::c_int = 3;
pub const CTL_NET: ::c_int = 4;
pub const CTL_DEBUG: ::c_int = 5;
pub const CTL_HW: ::c_int = 6;
pub const CTL_MACHDEP: ::c_int = 7;
pub const CTL_USER: ::c_int = 8;
pub const CTL_P1003_1B: ::c_int = 9;
pub const CTL_LWKT: ::c_int = 10;
pub const CTL_MAXID: ::c_int = 11;
pub const KERN_OSTYPE: ::c_int = 1;
pub const KERN_OSRELEASE: ::c_int = 2;
pub const KERN_OSREV: ::c_int = 3;
pub const KERN_VERSION: ::c_int = 4;
pub const KERN_MAXVNODES: ::c_int = 5;
pub const KERN_MAXPROC: ::c_int = 6;
pub const KERN_MAXFILES: ::c_int = 7;
pub const KERN_ARGMAX: ::c_int = 8;
pub const KERN_SECURELVL: ::c_int = 9;
pub const KERN_HOSTNAME: ::c_int = 10;
pub const KERN_HOSTID: ::c_int = 11;
pub const KERN_CLOCKRATE: ::c_int = 12;
pub const KERN_VNODE: ::c_int = 13;
pub const KERN_PROC: ::c_int = 14;
pub const KERN_FILE: ::c_int = 15;
pub const KERN_PROF: ::c_int = 16;
pub const KERN_POSIX1: ::c_int = 17;
pub const KERN_NGROUPS: ::c_int = 18;
pub const KERN_JOB_CONTROL: ::c_int = 19;
pub const KERN_SAVED_IDS: ::c_int = 20;
pub const KERN_BOOTTIME: ::c_int = 21;
pub const KERN_NISDOMAINNAME: ::c_int = 22;
pub const KERN_UPDATEINTERVAL: ::c_int = 23;
pub const KERN_OSRELDATE: ::c_int = 24;
pub const KERN_NTP_PLL: ::c_int = 25;
pub const KERN_BOOTFILE: ::c_int = 26;
pub const KERN_MAXFILESPERPROC: ::c_int = 27;
pub const KERN_MAXPROCPERUID: ::c_int = 28;
pub const KERN_DUMPDEV: ::c_int = 29;
pub const KERN_IPC: ::c_int = 30;
pub const KERN_DUMMY: ::c_int = 31;
pub const KERN_PS_STRINGS: ::c_int = 32;
pub const KERN_USRSTACK: ::c_int = 33;
pub const KERN_LOGSIGEXIT: ::c_int = 34;
pub const KERN_IOV_MAX: ::c_int = 35;
pub const KERN_MAXPOSIXLOCKSPERUID: ::c_int = 36;
pub const KERN_MAXID: ::c_int = 37;
pub const KERN_PROC_ALL: ::c_int = 0;
pub const KERN_PROC_PID: ::c_int = 1;
pub const KERN_PROC_PGRP: ::c_int = 2;
pub const KERN_PROC_SESSION: ::c_int = 3;
pub const KERN_PROC_TTY: ::c_int = 4;
pub const KERN_PROC_UID: ::c_int = 5;
pub const KERN_PROC_RUID: ::c_int = 6;
pub const KERN_PROC_ARGS: ::c_int = 7;
pub const KERN_PROC_CWD: ::c_int = 8;
pub const KERN_PROC_PATHNAME: ::c_int = 9;
pub const KERN_PROC_FLAGMASK: ::c_int = 0x10;
pub const KERN_PROC_FLAG_LWP: ::c_int = 0x10;
pub const KIPC_MAXSOCKBUF: ::c_int = 1;
pub const KIPC_SOCKBUF_WASTE: ::c_int = 2;
pub const KIPC_SOMAXCONN: ::c_int = 3;
pub const KIPC_MAX_LINKHDR: ::c_int = 4;
pub const KIPC_MAX_PROTOHDR: ::c_int = 5;
pub const KIPC_MAX_HDR: ::c_int = 6;
pub const KIPC_MAX_DATALEN: ::c_int = 7;
pub const KIPC_MBSTAT: ::c_int = 8;
pub const KIPC_NMBCLUSTERS: ::c_int = 9;
pub const HW_MACHINE: ::c_int = 1;
pub const HW_MODEL: ::c_int = 2;
pub const HW_NCPU: ::c_int = 3;
pub const HW_BYTEORDER: ::c_int = 4;
pub const HW_PHYSMEM: ::c_int = 5;
pub const HW_USERMEM: ::c_int = 6;
pub const HW_PAGESIZE: ::c_int = 7;
pub const HW_DISKNAMES: ::c_int = 8;
pub const HW_DISKSTATS: ::c_int = 9;
pub const HW_FLOATINGPT: ::c_int = 10;
pub const HW_MACHINE_ARCH: ::c_int = 11;
pub const HW_MACHINE_PLATFORM: ::c_int = 12;
pub const HW_SENSORS: ::c_int = 13;
pub const HW_MAXID: ::c_int = 14;
pub const USER_CS_PATH: ::c_int = 1;
pub const USER_BC_BASE_MAX: ::c_int = 2;
pub const USER_BC_DIM_MAX: ::c_int = 3;
pub const USER_BC_SCALE_MAX: ::c_int = 4;
pub const USER_BC_STRING_MAX: ::c_int = 5;
pub const USER_COLL_WEIGHTS_MAX: ::c_int = 6;
pub const USER_EXPR_NEST_MAX: ::c_int = 7;
pub const USER_LINE_MAX: ::c_int = 8;
pub const USER_RE_DUP_MAX: ::c_int = 9;
pub const USER_POSIX2_VERSION: ::c_int = 10;
pub const USER_POSIX2_C_BIND: ::c_int = 11;
pub const USER_POSIX2_C_DEV: ::c_int = 12;
pub const USER_POSIX2_CHAR_TERM: ::c_int = 13;
pub const USER_POSIX2_FORT_DEV: ::c_int = 14;
pub const USER_POSIX2_FORT_RUN: ::c_int = 15;
pub const USER_POSIX2_LOCALEDEF: ::c_int = 16;
pub const USER_POSIX2_SW_DEV: ::c_int = 17;
pub const USER_POSIX2_UPE: ::c_int = 18;
pub const USER_STREAM_MAX: ::c_int = 19;
pub const USER_TZNAME_MAX: ::c_int = 20;
pub const USER_MAXID: ::c_int = 21;
pub const CTL_P1003_1B_ASYNCHRONOUS_IO: ::c_int = 1;
pub const CTL_P1003_1B_MAPPED_FILES: ::c_int = 2;
pub const CTL_P1003_1B_MEMLOCK: ::c_int = 3;
pub const CTL_P1003_1B_MEMLOCK_RANGE: ::c_int = 4;
pub const CTL_P1003_1B_MEMORY_PROTECTION: ::c_int = 5;
pub const CTL_P1003_1B_MESSAGE_PASSING: ::c_int = 6;
pub const CTL_P1003_1B_PRIORITIZED_IO: ::c_int = 7;
pub const CTL_P1003_1B_PRIORITY_SCHEDULING: ::c_int = 8;
pub const CTL_P1003_1B_REALTIME_SIGNALS: ::c_int = 9;
pub const CTL_P1003_1B_SEMAPHORES: ::c_int = 10;
pub const CTL_P1003_1B_FSYNC: ::c_int = 11;
pub const CTL_P1003_1B_SHARED_MEMORY_OBJECTS: ::c_int = 12;
pub const CTL_P1003_1B_SYNCHRONIZED_IO: ::c_int = 13;
pub const CTL_P1003_1B_TIMERS: ::c_int = 14;
pub const CTL_P1003_1B_AIO_LISTIO_MAX: ::c_int = 15;
pub const CTL_P1003_1B_AIO_MAX: ::c_int = 16;
pub const CTL_P1003_1B_AIO_PRIO_DELTA_MAX: ::c_int = 17;
pub const CTL_P1003_1B_DELAYTIMER_MAX: ::c_int = 18;
pub const CTL_P1003_1B_UNUSED1: ::c_int = 19;
pub const CTL_P1003_1B_PAGESIZE: ::c_int = 20;
pub const CTL_P1003_1B_RTSIG_MAX: ::c_int = 21;
pub const CTL_P1003_1B_SEM_NSEMS_MAX: ::c_int = 22;
pub const CTL_P1003_1B_SEM_VALUE_MAX: ::c_int = 23;
pub const CTL_P1003_1B_SIGQUEUE_MAX: ::c_int = 24;
pub const CTL_P1003_1B_TIMER_MAX: ::c_int = 25;
pub const CTL_P1003_1B_MAXID: ::c_int = 26;

pub const EVFILT_READ: ::int16_t = -1;
pub const EVFILT_WRITE: ::int16_t = -2;
pub const EVFILT_AIO: ::int16_t = -3;
pub const EVFILT_VNODE: ::int16_t = -4;
pub const EVFILT_PROC: ::int16_t = -5;
pub const EVFILT_SIGNAL: ::int16_t = -6;
pub const EVFILT_TIMER: ::int16_t = -7;
pub const EVFILT_EXCEPT: ::int16_t = -8;
pub const EVFILT_USER: ::int16_t = -9;
pub const EVFILT_FS: ::int16_t = -10;

pub const EV_ADD: ::uint16_t = 0x1;
pub const EV_DELETE: ::uint16_t = 0x2;
pub const EV_ENABLE: ::uint16_t = 0x4;
pub const EV_DISABLE: ::uint16_t = 0x8;
pub const EV_ONESHOT: ::uint16_t = 0x10;
pub const EV_CLEAR: ::uint16_t = 0x20;
pub const EV_RECEIPT: ::uint16_t = 0x40;
pub const EV_DISPATCH: ::uint16_t = 0x80;
pub const EV_NODATA: ::uint16_t = 0x1000;
pub const EV_FLAG1: ::uint16_t = 0x2000;
pub const EV_ERROR: ::uint16_t = 0x4000;
pub const EV_EOF: ::uint16_t = 0x8000;
pub const EV_SYSFLAGS: ::uint16_t = 0xf000;

pub const NOTE_TRIGGER: ::uint32_t = 0x01000000;
pub const NOTE_FFNOP: ::uint32_t = 0x00000000;
pub const NOTE_FFAND: ::uint32_t = 0x40000000;
pub const NOTE_FFOR: ::uint32_t = 0x80000000;
pub const NOTE_FFCOPY: ::uint32_t = 0xc0000000;
pub const NOTE_FFCTRLMASK: ::uint32_t = 0xc0000000;
pub const NOTE_FFLAGSMASK: ::uint32_t = 0x00ffffff;
pub const NOTE_LOWAT: ::uint32_t = 0x00000001;
pub const NOTE_OOB: ::uint32_t = 0x00000002;
pub const NOTE_DELETE: ::uint32_t = 0x00000001;
pub const NOTE_WRITE: ::uint32_t = 0x00000002;
pub const NOTE_EXTEND: ::uint32_t = 0x00000004;
pub const NOTE_ATTRIB: ::uint32_t = 0x00000008;
pub const NOTE_LINK: ::uint32_t = 0x00000010;
pub const NOTE_RENAME: ::uint32_t = 0x00000020;
pub const NOTE_REVOKE: ::uint32_t = 0x00000040;
pub const NOTE_EXIT: ::uint32_t = 0x80000000;
pub const NOTE_FORK: ::uint32_t = 0x40000000;
pub const NOTE_EXEC: ::uint32_t = 0x20000000;
pub const NOTE_PDATAMASK: ::uint32_t = 0x000fffff;
pub const NOTE_PCTRLMASK: ::uint32_t = 0xf0000000;
pub const NOTE_TRACK: ::uint32_t = 0x00000001;
pub const NOTE_TRACKERR: ::uint32_t = 0x00000002;
pub const NOTE_CHILD: ::uint32_t = 0x00000004;

pub const SO_SNDSPACE: ::c_int = 0x100a;
pub const SO_CPUHINT: ::c_int = 0x1030;

// https://github.com/DragonFlyBSD/DragonFlyBSD/blob/master/sys/net/if.h#L101
pub const IFF_UP: ::c_int = 0x1; // interface is up
pub const IFF_BROADCAST: ::c_int = 0x2; // broadcast address valid
pub const IFF_DEBUG: ::c_int = 0x4; // turn on debugging
pub const IFF_LOOPBACK: ::c_int = 0x8; // is a loopback net
pub const IFF_POINTOPOINT: ::c_int = 0x10; // interface is point-to-point link
pub const IFF_SMART: ::c_int = 0x20; // interface manages own routes
pub const IFF_RUNNING: ::c_int = 0x40; // resources allocated
pub const IFF_NOARP: ::c_int = 0x80; // no address resolution protocol
pub const IFF_PROMISC: ::c_int = 0x100; // receive all packets
pub const IFF_ALLMULTI: ::c_int = 0x200; // receive all multicast packets
pub const IFF_OACTIVE_COMPAT: ::c_int = 0x400; // was transmission in progress
pub const IFF_SIMPLEX: ::c_int = 0x800; // can't hear own transmissions
pub const IFF_LINK0: ::c_int = 0x1000; // per link layer defined bit
pub const IFF_LINK1: ::c_int = 0x2000; // per link layer defined bit
pub const IFF_LINK2: ::c_int = 0x4000; // per link layer defined bit
pub const IFF_ALTPHYS: ::c_int = IFF_LINK2; // use alternate physical connection
pub const IFF_MULTICAST: ::c_int = 0x8000; // supports multicast
// was interface is in polling mode
pub const IFF_POLLING_COMPAT: ::c_int = 0x10000;
pub const IFF_PPROMISC: ::c_int = 0x20000; // user-requested promisc mode
pub const IFF_MONITOR: ::c_int = 0x40000; // user-requested monitor mode
pub const IFF_STATICARP: ::c_int = 0x80000; // static ARP
pub const IFF_NPOLLING: ::c_int = 0x100000; // interface is in polling mode
pub const IFF_IDIRECT: ::c_int = 0x200000; // direct input

//
// sys/netinet/in.h
// Protocols (RFC 1700)
// NOTE: These are in addition to the constants defined in src/unix/mod.rs

// IPPROTO_IP defined in src/unix/mod.rs
/// IP6 hop-by-hop options
pub const IPPROTO_HOPOPTS: ::c_int = 0;
// IPPROTO_ICMP defined in src/unix/mod.rs
/// group mgmt protocol
pub const IPPROTO_IGMP: ::c_int = 2;
/// gateway^2 (deprecated)
pub const IPPROTO_GGP: ::c_int = 3;
/// for compatibility
pub const IPPROTO_IPIP: ::c_int = 4;
// IPPROTO_TCP defined in src/unix/mod.rs
/// Stream protocol II.
pub const IPPROTO_ST: ::c_int = 7;
/// exterior gateway protocol
pub const IPPROTO_EGP: ::c_int = 8;
/// private interior gateway
pub const IPPROTO_PIGP: ::c_int = 9;
/// BBN RCC Monitoring
pub const IPPROTO_RCCMON: ::c_int = 10;
/// network voice protocol
pub const IPPROTO_NVPII: ::c_int = 11;
/// pup
pub const IPPROTO_PUP: ::c_int = 12;
/// Argus
pub const IPPROTO_ARGUS: ::c_int = 13;
/// EMCON
pub const IPPROTO_EMCON: ::c_int = 14;
/// Cross Net Debugger
pub const IPPROTO_XNET: ::c_int = 15;
/// Chaos
pub const IPPROTO_CHAOS: ::c_int = 16;
// IPPROTO_UDP defined in src/unix/mod.rs
/// Multiplexing
pub const IPPROTO_MUX: ::c_int = 18;
/// DCN Measurement Subsystems
pub const IPPROTO_MEAS: ::c_int = 19;
/// Host Monitoring
pub const IPPROTO_HMP: ::c_int = 20;
/// Packet Radio Measurement
pub const IPPROTO_PRM: ::c_int = 21;
/// xns idp
pub const IPPROTO_IDP: ::c_int = 22;
/// Trunk-1
pub const IPPROTO_TRUNK1: ::c_int = 23;
/// Trunk-2
pub const IPPROTO_TRUNK2: ::c_int = 24;
/// Leaf-1
pub const IPPROTO_LEAF1: ::c_int = 25;
/// Leaf-2
pub const IPPROTO_LEAF2: ::c_int = 26;
/// Reliable Data
pub const IPPROTO_RDP: ::c_int = 27;
/// Reliable Transaction
pub const IPPROTO_IRTP: ::c_int = 28;
/// tp-4 w/ class negotiation
pub const IPPROTO_TP: ::c_int = 29;
/// Bulk Data Transfer
pub const IPPROTO_BLT: ::c_int = 30;
/// Network Services
pub const IPPROTO_NSP: ::c_int = 31;
/// Merit Internodal
pub const IPPROTO_INP: ::c_int = 32;
/// Sequential Exchange
pub const IPPROTO_SEP: ::c_int = 33;
/// Third Party Connect
pub const IPPROTO_3PC: ::c_int = 34;
/// InterDomain Policy Routing
pub const IPPROTO_IDPR: ::c_int = 35;
/// XTP
pub const IPPROTO_XTP: ::c_int = 36;
/// Datagram Delivery
pub const IPPROTO_DDP: ::c_int = 37;
/// Control Message Transport
pub const IPPROTO_CMTP: ::c_int = 38;
/// TP++ Transport
pub const IPPROTO_TPXX: ::c_int = 39;
/// IL transport protocol
pub const IPPROTO_IL: ::c_int = 40;
// IPPROTO_IPV6 defined in src/unix/mod.rs
/// Source Demand Routing
pub const IPPROTO_SDRP: ::c_int = 42;
/// IP6 routing header
pub const IPPROTO_ROUTING: ::c_int = 43;
/// IP6 fragmentation header
pub const IPPROTO_FRAGMENT: ::c_int = 44;
/// InterDomain Routing
pub const IPPROTO_IDRP: ::c_int = 45;
/// resource reservation
pub const IPPROTO_RSVP: ::c_int = 46;
/// General Routing Encap.
pub const IPPROTO_GRE: ::c_int = 47;
/// Mobile Host Routing
pub const IPPROTO_MHRP: ::c_int = 48;
/// BHA
pub const IPPROTO_BHA: ::c_int = 49;
/// IP6 Encap Sec. Payload
pub const IPPROTO_ESP: ::c_int = 50;
/// IP6 Auth Header
pub const IPPROTO_AH: ::c_int = 51;
/// Integ. Net Layer Security
pub const IPPROTO_INLSP: ::c_int = 52;
/// IP with encryption
pub const IPPROTO_SWIPE: ::c_int = 53;
/// Next Hop Resolution
pub const IPPROTO_NHRP: ::c_int = 54;
/// IP Mobility
pub const IPPROTO_MOBILE: ::c_int = 55;
/// Transport Layer Security
pub const IPPROTO_TLSP: ::c_int = 56;
/// SKIP
pub const IPPROTO_SKIP: ::c_int = 57;
// IPPROTO_ICMPV6 defined in src/unix/mod.rs
/// IP6 no next header
pub const IPPROTO_NONE: ::c_int = 59;
/// IP6 destination option
pub const IPPROTO_DSTOPTS: ::c_int = 60;
/// any host internal protocol
pub const IPPROTO_AHIP: ::c_int = 61;
/// CFTP
pub const IPPROTO_CFTP: ::c_int = 62;
/// "hello" routing protocol
pub const IPPROTO_HELLO: ::c_int = 63;
/// SATNET/Backroom EXPAK
pub const IPPROTO_SATEXPAK: ::c_int = 64;
/// Kryptolan
pub const IPPROTO_KRYPTOLAN: ::c_int = 65;
/// Remote Virtual Disk
pub const IPPROTO_RVD: ::c_int = 66;
/// Pluribus Packet Core
pub const IPPROTO_IPPC: ::c_int = 67;
/// Any distributed FS
pub const IPPROTO_ADFS: ::c_int = 68;
/// Satnet Monitoring
pub const IPPROTO_SATMON: ::c_int = 69;
/// VISA Protocol
pub const IPPROTO_VISA: ::c_int = 70;
/// Packet Core Utility
pub const IPPROTO_IPCV: ::c_int = 71;
/// Comp. Prot. Net. Executive
pub const IPPROTO_CPNX: ::c_int = 72;
/// Comp. Prot. HeartBeat
pub const IPPROTO_CPHB: ::c_int = 73;
/// Wang Span Network
pub const IPPROTO_WSN: ::c_int = 74;
/// Packet Video Protocol
pub const IPPROTO_PVP: ::c_int = 75;
/// BackRoom SATNET Monitoring
pub const IPPROTO_BRSATMON: ::c_int = 76;
/// Sun net disk proto (temp.)
pub const IPPROTO_ND: ::c_int = 77;
/// WIDEBAND Monitoring
pub const IPPROTO_WBMON: ::c_int = 78;
/// WIDEBAND EXPAK
pub const IPPROTO_WBEXPAK: ::c_int = 79;
/// ISO cnlp
pub const IPPROTO_EON: ::c_int = 80;
/// VMTP
pub const IPPROTO_VMTP: ::c_int = 81;
/// Secure VMTP
pub const IPPROTO_SVMTP: ::c_int = 82;
/// Banyon VINES
pub const IPPROTO_VINES: ::c_int = 83;
/// TTP
pub const IPPROTO_TTP: ::c_int = 84;
/// NSFNET-IGP
pub const IPPROTO_IGP: ::c_int = 85;
/// dissimilar gateway prot.
pub const IPPROTO_DGP: ::c_int = 86;
/// TCF
pub const IPPROTO_TCF: ::c_int = 87;
/// Cisco/GXS IGRP
pub const IPPROTO_IGRP: ::c_int = 88;
/// OSPFIGP
pub const IPPROTO_OSPFIGP: ::c_int = 89;
/// Strite RPC protocol
pub const IPPROTO_SRPC: ::c_int = 90;
/// Locus Address Resoloution
pub const IPPROTO_LARP: ::c_int = 91;
/// Multicast Transport
pub const IPPROTO_MTP: ::c_int = 92;
/// AX.25 Frames
pub const IPPROTO_AX25: ::c_int = 93;
/// IP encapsulated in IP
pub const IPPROTO_IPEIP: ::c_int = 94;
/// Mobile Int.ing control
pub const IPPROTO_MICP: ::c_int = 95;
/// Semaphore Comm. security
pub const IPPROTO_SCCSP: ::c_int = 96;
/// Ethernet IP encapsulation
pub const IPPROTO_ETHERIP: ::c_int = 97;
/// encapsulation header
pub const IPPROTO_ENCAP: ::c_int = 98;
/// any private encr. scheme
pub const IPPROTO_APES: ::c_int = 99;
/// GMTP
pub const IPPROTO_GMTP: ::c_int = 100;
/// payload compression (IPComp)
pub const IPPROTO_IPCOMP: ::c_int = 108;

/* 101-254: Partly Unassigned */
/// Protocol Independent Mcast
pub const IPPROTO_PIM: ::c_int = 103;
/// CARP
pub const IPPROTO_CARP: ::c_int = 112;
/// PGM
pub const IPPROTO_PGM: ::c_int = 113;
/// PFSYNC
pub const IPPROTO_PFSYNC: ::c_int = 240;

/* 255: Reserved */
/* BSD Private, local use, namespace incursion, no longer used */
/// divert pseudo-protocol
pub const IPPROTO_DIVERT: ::c_int = 254;
pub const IPPROTO_MAX: ::c_int = 256;
/// last return value of *_input(), meaning "all job for this pkt is done".
pub const IPPROTO_DONE: ::c_int = 257;

/// Used by RSS: the layer3 protocol is unknown
pub const IPPROTO_UNKNOWN: ::c_int = 258;

pub const AF_BLUETOOTH: ::c_int = 33;
pub const AF_MPLS: ::c_int = 34;
pub const AF_IEEE80211: ::c_int = 35;
pub const AF_MAX: ::c_int = 36;

pub const PF_BLUETOOTH: ::c_int = AF_BLUETOOTH;
pub const PF_MAX: ::c_int = AF_MAX;

pub const NET_RT_DUMP: ::c_int = 1;
pub const NET_RT_FLAGS: ::c_int = 2;
pub const NET_RT_IFLIST: ::c_int = 3;
pub const NET_RT_MAXID: ::c_int = 4;

pub const SOMAXOPT_SIZE: ::c_int = 65536;

#[doc(hidden)]
pub const NET_MAXID: ::c_int = AF_MAX;

pub const MSG_UNUSED09: ::c_int = 0x00000200;
pub const MSG_NOSIGNAL: ::c_int = 0x00000400;
pub const MSG_SYNC: ::c_int = 0x00000800;
pub const MSG_CMSG_CLOEXEC: ::c_int = 0x00001000;
pub const MSG_FBLOCKING: ::c_int = 0x00010000;
pub const MSG_FNONBLOCKING: ::c_int = 0x00020000;
pub const MSG_FMASK: ::c_int = 0xFFFF0000;

pub const EMPTY: ::c_short = 0;
pub const RUN_LVL: ::c_short = 1;
pub const BOOT_TIME: ::c_short = 2;
pub const OLD_TIME: ::c_short = 3;
pub const NEW_TIME: ::c_short = 4;
pub const INIT_PROCESS: ::c_short = 5;
pub const LOGIN_PROCESS: ::c_short = 6;
pub const USER_PROCESS: ::c_short = 7;
pub const DEAD_PROCESS: ::c_short = 8;

pub const LC_COLLATE_MASK: ::c_int = (1 << 0);
pub const LC_CTYPE_MASK: ::c_int = (1 << 1);
pub const LC_MONETARY_MASK: ::c_int = (1 << 2);
pub const LC_NUMERIC_MASK: ::c_int = (1 << 3);
pub const LC_TIME_MASK: ::c_int = (1 << 4);
pub const LC_MESSAGES_MASK: ::c_int = (1 << 5);
pub const LC_ALL_MASK: ::c_int = LC_COLLATE_MASK
                               | LC_CTYPE_MASK
                               | LC_MESSAGES_MASK
                               | LC_MONETARY_MASK
                               | LC_NUMERIC_MASK
                               | LC_TIME_MASK;

pub const TIOCSIG: ::c_uint = 0x2000745f;
pub const BTUARTDISC: ::c_int = 0x7;
pub const TIOCDCDTIMESTAMP: ::c_uint = 0x40107458;
pub const TIOCISPTMASTER: ::c_uint = 0x20007455;
pub const TIOCMODG: ::c_uint = 0x40047403;
pub const TIOCMODS: ::c_ulong = 0x80047404;
pub const TIOCREMOTE: ::c_ulong = 0x80047469;

// Constants used by "at" family of system calls.
pub const AT_FDCWD:            ::c_int = 0xFFFAFDCD; // invalid file descriptor
pub const AT_SYMLINK_NOFOLLOW: ::c_int = 1;
pub const AT_REMOVEDIR:        ::c_int = 2;
pub const AT_EACCESS:          ::c_int = 4;
pub const AT_SYMLINK_FOLLOW:   ::c_int = 8;

pub const VCHECKPT: usize = 19;

pub const _PC_2_SYMLINKS: ::c_int = 22;
pub const _PC_TIMESTAMP_RESOLUTION: ::c_int = 23;

pub const _SC_V7_ILP32_OFF32: ::c_int = 122;
pub const _SC_V7_ILP32_OFFBIG: ::c_int = 123;
pub const _SC_V7_LP64_OFF64: ::c_int = 124;
pub const _SC_V7_LPBIG_OFFBIG: ::c_int = 125;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: ::c_int = 126;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: ::c_int = 127;

pub const WCONTINUED: ::c_int = 4;
pub const WSTOPPED: ::c_int = 0o177;

extern {
    pub fn mprotect(addr: *mut ::c_void, len: ::size_t, prot: ::c_int)
                    -> ::c_int;
    pub fn clock_getres(clk_id: ::clockid_t, tp: *mut ::timespec) -> ::c_int;
    pub fn clock_gettime(clk_id: ::clockid_t, tp: *mut ::timespec) -> ::c_int;
    pub fn clock_settime(clk_id: ::clockid_t, tp: *const ::timespec) -> ::c_int;

    pub fn setutxdb(_type: ::c_uint, file: *mut ::c_char) -> ::c_int;

    pub fn aio_waitcomplete(iocbp: *mut *mut aiocb,
                            timeout: *mut ::timespec) -> ::c_int;

    pub fn freelocale(loc: ::locale_t);
}
