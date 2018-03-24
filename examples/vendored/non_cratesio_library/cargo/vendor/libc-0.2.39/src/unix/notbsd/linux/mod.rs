//! Linux-specific definitions for linux-like values

use dox::{mem, Option};

pub type useconds_t = u32;
pub type dev_t = u64;
pub type socklen_t = u32;
pub type pthread_t = c_ulong;
pub type mode_t = u32;
pub type ino64_t = u64;
pub type off64_t = i64;
pub type blkcnt64_t = i64;
pub type rlim64_t = u64;
pub type mqd_t = ::c_int;
pub type nfds_t = ::c_ulong;
pub type nl_item = ::c_int;
pub type idtype_t = ::c_uint;
pub type loff_t = ::c_longlong;

pub type __u8 = ::c_uchar;
pub type __u16 = ::c_ushort;
pub type __s16 = ::c_short;
pub type __u32 = ::c_uint;
pub type __s32 = ::c_int;

pub type Elf32_Half = u16;
pub type Elf32_Word = u32;
pub type Elf32_Off = u32;
pub type Elf32_Addr = u32;

pub type Elf64_Half = u16;
pub type Elf64_Word = u32;
pub type Elf64_Off = u64;
pub type Elf64_Addr = u64;
pub type Elf64_Xword = u64;

pub enum fpos64_t {} // TODO: fill this out with a struct

s! {
    pub struct dirent {
        pub d_ino: ::ino_t,
        pub d_off: ::off_t,
        pub d_reclen: ::c_ushort,
        pub d_type: ::c_uchar,
        pub d_name: [::c_char; 256],
    }

    pub struct dirent64 {
        pub d_ino: ::ino64_t,
        pub d_off: ::off64_t,
        pub d_reclen: ::c_ushort,
        pub d_type: ::c_uchar,
        pub d_name: [::c_char; 256],
    }

    pub struct rlimit64 {
        pub rlim_cur: rlim64_t,
        pub rlim_max: rlim64_t,
    }

    pub struct glob_t {
        pub gl_pathc: ::size_t,
        pub gl_pathv: *mut *mut c_char,
        pub gl_offs: ::size_t,
        pub gl_flags: ::c_int,

        __unused1: *mut ::c_void,
        __unused2: *mut ::c_void,
        __unused3: *mut ::c_void,
        __unused4: *mut ::c_void,
        __unused5: *mut ::c_void,
    }

    pub struct ifaddrs {
        pub ifa_next: *mut ifaddrs,
        pub ifa_name: *mut c_char,
        pub ifa_flags: ::c_uint,
        pub ifa_addr: *mut ::sockaddr,
        pub ifa_netmask: *mut ::sockaddr,
        pub ifa_ifu: *mut ::sockaddr, // FIXME This should be a union
        pub ifa_data: *mut ::c_void
    }

    pub struct pthread_mutex_t {
        #[cfg(any(target_arch = "mips",
                  target_arch = "arm",
                  target_arch = "powerpc",
                  all(target_arch = "x86_64",
                      target_pointer_width = "32")))]
        __align: [::c_long; 0],
        #[cfg(not(any(target_arch = "mips",
                      target_arch = "arm",
                      target_arch = "powerpc",
                      all(target_arch = "x86_64",
                          target_pointer_width = "32"))))]
        __align: [::c_longlong; 0],
        size: [u8; __SIZEOF_PTHREAD_MUTEX_T],
    }

    pub struct pthread_rwlock_t {
        #[cfg(any(target_arch = "mips",
                  target_arch = "arm",
                  target_arch = "powerpc",
                  all(target_arch = "x86_64",
                      target_pointer_width = "32")))]
        __align: [::c_long; 0],
        #[cfg(not(any(target_arch = "mips",
                      target_arch = "arm",
                      target_arch = "powerpc",
                      all(target_arch = "x86_64",
                          target_pointer_width = "32"))))]
        __align: [::c_longlong; 0],
        size: [u8; __SIZEOF_PTHREAD_RWLOCK_T],
    }

    pub struct pthread_mutexattr_t {
        #[cfg(any(target_arch = "x86_64", target_arch = "powerpc64",
                  target_arch = "mips64", target_arch = "s390x",
                  target_arch = "sparc64"))]
        __align: [::c_int; 0],
        #[cfg(not(any(target_arch = "x86_64", target_arch = "powerpc64",
                      target_arch = "mips64", target_arch = "s390x",
                      target_arch = "sparc64", target_arch = "aarch64")))]
        __align: [::c_long; 0],
        #[cfg(all(target_arch = "aarch64", target_env = "gnu"))]
        __align: [::c_long; 0],
        #[cfg(all(target_arch = "aarch64", target_env = "musl"))]
        __align: [::c_int; 0],
        size: [u8; __SIZEOF_PTHREAD_MUTEXATTR_T],
    }

    pub struct pthread_rwlockattr_t {
        #[cfg(any(target_env = "musl"))]
        __align: [::c_int; 0],
        #[cfg(not(any(target_env = "musl")))]
        __align: [::c_long; 0],
        size: [u8; __SIZEOF_PTHREAD_RWLOCKATTR_T],
    }

    pub struct pthread_cond_t {
        #[cfg(any(target_env = "musl"))]
        __align: [*const ::c_void; 0],
        #[cfg(not(any(target_env = "musl")))]
        __align: [::c_longlong; 0],
        size: [u8; __SIZEOF_PTHREAD_COND_T],
    }

    pub struct pthread_condattr_t {
        __align: [::c_int; 0],
        size: [u8; __SIZEOF_PTHREAD_CONDATTR_T],
    }

    pub struct passwd {
        pub pw_name: *mut ::c_char,
        pub pw_passwd: *mut ::c_char,
        pub pw_uid: ::uid_t,
        pub pw_gid: ::gid_t,
        pub pw_gecos: *mut ::c_char,
        pub pw_dir: *mut ::c_char,
        pub pw_shell: *mut ::c_char,
    }

    pub struct spwd {
        pub sp_namp: *mut ::c_char,
        pub sp_pwdp: *mut ::c_char,
        pub sp_lstchg: ::c_long,
        pub sp_min: ::c_long,
        pub sp_max: ::c_long,
        pub sp_warn: ::c_long,
        pub sp_inact: ::c_long,
        pub sp_expire: ::c_long,
        pub sp_flag: ::c_ulong,
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
        #[cfg(target_endian = "little")]
        pub f_fsid: ::c_ulong,
        #[cfg(all(target_pointer_width = "32", not(target_arch = "x86_64")))]
        __f_unused: ::c_int,
        #[cfg(target_endian = "big")]
        pub f_fsid: ::c_ulong,
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
        __f_spare: [::c_int; 6],
    }

    pub struct dqblk {
        pub dqb_bhardlimit: ::uint64_t,
        pub dqb_bsoftlimit: ::uint64_t,
        pub dqb_curspace: ::uint64_t,
        pub dqb_ihardlimit: ::uint64_t,
        pub dqb_isoftlimit: ::uint64_t,
        pub dqb_curinodes: ::uint64_t,
        pub dqb_btime: ::uint64_t,
        pub dqb_itime: ::uint64_t,
        pub dqb_valid: ::uint32_t,
    }

    pub struct signalfd_siginfo {
        pub ssi_signo: ::uint32_t,
        pub ssi_errno: ::int32_t,
        pub ssi_code: ::int32_t,
        pub ssi_pid: ::uint32_t,
        pub ssi_uid: ::uint32_t,
        pub ssi_fd: ::int32_t,
        pub ssi_tid: ::uint32_t,
        pub ssi_band: ::uint32_t,
        pub ssi_overrun: ::uint32_t,
        pub ssi_trapno: ::uint32_t,
        pub ssi_status: ::int32_t,
        pub ssi_int: ::int32_t,
        pub ssi_ptr: ::uint64_t,
        pub ssi_utime: ::uint64_t,
        pub ssi_stime: ::uint64_t,
        pub ssi_addr: ::uint64_t,
        _pad: [::uint8_t; 48],
    }

    pub struct itimerspec {
        pub it_interval: ::timespec,
        pub it_value: ::timespec,
    }

    pub struct fsid_t {
        __val: [::c_int; 2],
    }

    // x32 compatibility
    // See https://sourceware.org/bugzilla/show_bug.cgi?id=21279
    pub struct mq_attr {
        #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
        pub mq_flags: i64,
        #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
        pub mq_maxmsg: i64,
        #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
        pub mq_msgsize: i64,
        #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
        pub mq_curmsgs: i64,
        #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
        pad: [i64; 4],

        #[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))]
        pub mq_flags: ::c_long,
        #[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))]
        pub mq_maxmsg: ::c_long,
        #[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))]
        pub mq_msgsize: ::c_long,
        #[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))]
        pub mq_curmsgs: ::c_long,
        #[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))]
        pad: [::c_long; 4],
    }

    pub struct cpu_set_t {
        #[cfg(all(target_pointer_width = "32",
                  not(target_arch = "x86_64")))]
        bits: [u32; 32],
        #[cfg(not(all(target_pointer_width = "32",
                      not(target_arch = "x86_64"))))]
        bits: [u64; 16],
    }

    pub struct if_nameindex {
        pub if_index: ::c_uint,
        pub if_name: *mut ::c_char,
    }

    // System V IPC
    pub struct msginfo {
        pub msgpool: ::c_int,
        pub msgmap: ::c_int,
        pub msgmax: ::c_int,
        pub msgmnb: ::c_int,
        pub msgmni: ::c_int,
        pub msgssz: ::c_int,
        pub msgtql: ::c_int,
        pub msgseg: ::c_ushort,
    }

    pub struct mmsghdr {
        pub msg_hdr: ::msghdr,
        pub msg_len: ::c_uint,
    }

    pub struct sembuf {
        pub sem_num: ::c_ushort,
        pub sem_op: ::c_short,
        pub sem_flg: ::c_short,
    }

    pub struct input_event {
        pub time: ::timeval,
        pub type_: ::__u16,
        pub code: ::__u16,
        pub value: ::__s32,
    }

    pub struct input_id {
        pub bustype: ::__u16,
        pub vendor: ::__u16,
        pub product: ::__u16,
        pub version: ::__u16,
    }

    pub struct input_absinfo {
        pub value: ::__s32,
        pub minimum: ::__s32,
        pub maximum: ::__s32,
        pub fuzz: ::__s32,
        pub flat: ::__s32,
        pub resolution: ::__s32,
    }

    pub struct input_keymap_entry {
        pub flags: ::__u8,
        pub len: ::__u8,
        pub index: ::__u16,
        pub keycode: ::__u32,
        pub scancode: [::__u8; 32],
    }

    pub struct input_mask {
        pub type_: ::__u32,
        pub codes_size: ::__u32,
        pub codes_ptr: ::__u64,
    }

    pub struct ff_replay {
        pub length: ::__u16,
        pub delay: ::__u16,
    }

    pub struct ff_trigger {
        pub button: ::__u16,
        pub interval: ::__u16,
    }

    pub struct ff_envelope {
        pub attack_length: ::__u16,
        pub attack_level: ::__u16,
        pub fade_length: ::__u16,
        pub fade_level: ::__u16,
    }

    pub struct ff_constant_effect {
        pub level: ::__s16,
        pub envelope: ff_envelope,
    }

    pub struct ff_ramp_effect {
        pub start_level: ::__s16,
        pub end_level: ::__s16,
        pub envelope: ff_envelope,
    }

    pub struct ff_condition_effect {
        pub right_saturation: ::__u16,
        pub left_saturation: ::__u16,

        pub right_coeff: ::__s16,
        pub left_coeff: ::__s16,

        pub deadband: ::__u16,
        pub center: ::__s16,
    }

    pub struct ff_periodic_effect {
        pub waveform: ::__u16,
        pub period: ::__u16,
        pub magnitude: ::__s16,
        pub offset: ::__s16,
        pub phase: ::__u16,

        pub envelope: ff_envelope,

        pub custom_len: ::__u32,
        pub custom_data: *mut ::__s16,
    }

    pub struct ff_rumble_effect {
        pub strong_magnitude: ::__u16,
        pub weak_magnitude: ::__u16,
    }

    pub struct ff_effect {
        pub type_: ::__u16,
        pub id: ::__s16,
        pub direction: ::__u16,
        pub trigger: ff_trigger,
        pub replay: ff_replay,
        // FIXME this is actually a union
        #[cfg(target_pointer_width = "64")]
        pub u: [u64; 4],
        #[cfg(target_pointer_width = "32")]
        pub u: [u32; 7],
    }

    pub struct dl_phdr_info {
        #[cfg(target_pointer_width = "64")]
        pub dlpi_addr: Elf64_Addr,
        #[cfg(target_pointer_width = "32")]
        pub dlpi_addr: Elf32_Addr,

        pub dlpi_name: *const ::c_char,

        #[cfg(target_pointer_width = "64")]
        pub dlpi_phdr: *const Elf64_Phdr,
        #[cfg(target_pointer_width = "32")]
        pub dlpi_phdr: *const Elf32_Phdr,

        #[cfg(target_pointer_width = "64")]
        pub dlpi_phnum: Elf64_Half,
        #[cfg(target_pointer_width = "32")]
        pub dlpi_phnum: Elf32_Half,

        pub dlpi_adds: ::c_ulonglong,
        pub dlpi_subs: ::c_ulonglong,
        pub dlpi_tls_modid: ::size_t,
        pub dlpi_tls_data: *mut ::c_void,
    }

    pub struct Elf32_Phdr {
        pub p_type: Elf32_Word,
        pub p_offset: Elf32_Off,
        pub p_vaddr: Elf32_Addr,
        pub p_paddr: Elf32_Addr,
        pub p_filesz: Elf32_Word,
        pub p_memsz: Elf32_Word,
        pub p_flags: Elf32_Word,
        pub p_align: Elf32_Word,
    }

    pub struct Elf64_Phdr {
        pub p_type: Elf64_Word,
        pub p_flags: Elf64_Word,
        pub p_offset: Elf64_Off,
        pub p_vaddr: Elf64_Addr,
        pub p_paddr: Elf64_Addr,
        pub p_filesz: Elf64_Xword,
        pub p_memsz: Elf64_Xword,
        pub p_align: Elf64_Xword,
    }

    pub struct ucred {
        pub pid: ::pid_t,
        pub uid: ::uid_t,
        pub gid: ::gid_t,
    }

    pub struct mntent {
        pub mnt_fsname: *mut ::c_char,
        pub mnt_dir: *mut ::c_char,
        pub mnt_type: *mut ::c_char,
        pub mnt_opts: *mut ::c_char,
        pub mnt_freq: ::c_int,
        pub mnt_passno: ::c_int,
    }

    pub struct posix_spawn_file_actions_t {
        __allocated: ::c_int,
        __used: ::c_int,
        __actions: *mut ::c_int,
        __pad: [::c_int; 16],
    }

    pub struct posix_spawnattr_t {
        __flags: ::c_short,
        __pgrp: ::pid_t,
        __sd: ::sigset_t,
        __ss: ::sigset_t,
        #[cfg(target_env = "musl")]
        __prio: ::c_int,
        #[cfg(not(target_env = "musl"))]
        __sp: ::sched_param,
        __policy: ::c_int,
        __pad: [::c_int; 16],
    }

    pub struct genlmsghdr {
        cmd: u8,
        version: u8,
        reserved: u16,
    }
}

pub const ABDAY_1: ::nl_item = 0x20000;
pub const ABDAY_2: ::nl_item = 0x20001;
pub const ABDAY_3: ::nl_item = 0x20002;
pub const ABDAY_4: ::nl_item = 0x20003;
pub const ABDAY_5: ::nl_item = 0x20004;
pub const ABDAY_6: ::nl_item = 0x20005;
pub const ABDAY_7: ::nl_item = 0x20006;

pub const DAY_1: ::nl_item = 0x20007;
pub const DAY_2: ::nl_item = 0x20008;
pub const DAY_3: ::nl_item = 0x20009;
pub const DAY_4: ::nl_item = 0x2000A;
pub const DAY_5: ::nl_item = 0x2000B;
pub const DAY_6: ::nl_item = 0x2000C;
pub const DAY_7: ::nl_item = 0x2000D;

pub const ABMON_1: ::nl_item = 0x2000E;
pub const ABMON_2: ::nl_item = 0x2000F;
pub const ABMON_3: ::nl_item = 0x20010;
pub const ABMON_4: ::nl_item = 0x20011;
pub const ABMON_5: ::nl_item = 0x20012;
pub const ABMON_6: ::nl_item = 0x20013;
pub const ABMON_7: ::nl_item = 0x20014;
pub const ABMON_8: ::nl_item = 0x20015;
pub const ABMON_9: ::nl_item = 0x20016;
pub const ABMON_10: ::nl_item = 0x20017;
pub const ABMON_11: ::nl_item = 0x20018;
pub const ABMON_12: ::nl_item = 0x20019;

pub const MON_1: ::nl_item = 0x2001A;
pub const MON_2: ::nl_item = 0x2001B;
pub const MON_3: ::nl_item = 0x2001C;
pub const MON_4: ::nl_item = 0x2001D;
pub const MON_5: ::nl_item = 0x2001E;
pub const MON_6: ::nl_item = 0x2001F;
pub const MON_7: ::nl_item = 0x20020;
pub const MON_8: ::nl_item = 0x20021;
pub const MON_9: ::nl_item = 0x20022;
pub const MON_10: ::nl_item = 0x20023;
pub const MON_11: ::nl_item = 0x20024;
pub const MON_12: ::nl_item = 0x20025;

pub const AM_STR: ::nl_item = 0x20026;
pub const PM_STR: ::nl_item = 0x20027;

pub const D_T_FMT: ::nl_item = 0x20028;
pub const D_FMT: ::nl_item = 0x20029;
pub const T_FMT: ::nl_item = 0x2002A;
pub const T_FMT_AMPM: ::nl_item = 0x2002B;

pub const ERA: ::nl_item = 0x2002C;
pub const ERA_D_FMT: ::nl_item = 0x2002E;
pub const ALT_DIGITS: ::nl_item = 0x2002F;
pub const ERA_D_T_FMT: ::nl_item = 0x20030;
pub const ERA_T_FMT: ::nl_item = 0x20031;

pub const CODESET: ::nl_item = 14;

pub const CRNCYSTR: ::nl_item = 0x4000F;

pub const RUSAGE_THREAD: ::c_int = 1;
pub const RUSAGE_CHILDREN: ::c_int = -1;

pub const RADIXCHAR: ::nl_item = 0x10000;
pub const THOUSEP: ::nl_item = 0x10001;

pub const YESEXPR: ::nl_item = 0x50000;
pub const NOEXPR: ::nl_item = 0x50001;
pub const YESSTR: ::nl_item = 0x50002;
pub const NOSTR: ::nl_item = 0x50003;

pub const FILENAME_MAX: ::c_uint = 4096;
pub const L_tmpnam: ::c_uint = 20;
pub const _PC_LINK_MAX: ::c_int = 0;
pub const _PC_MAX_CANON: ::c_int = 1;
pub const _PC_MAX_INPUT: ::c_int = 2;
pub const _PC_NAME_MAX: ::c_int = 3;
pub const _PC_PATH_MAX: ::c_int = 4;
pub const _PC_PIPE_BUF: ::c_int = 5;
pub const _PC_CHOWN_RESTRICTED: ::c_int = 6;
pub const _PC_NO_TRUNC: ::c_int = 7;
pub const _PC_VDISABLE: ::c_int = 8;
pub const _PC_SYNC_IO: ::c_int = 9;
pub const _PC_ASYNC_IO: ::c_int = 10;
pub const _PC_PRIO_IO: ::c_int = 11;
pub const _PC_SOCK_MAXBUF: ::c_int = 12;
pub const _PC_FILESIZEBITS: ::c_int = 13;
pub const _PC_REC_INCR_XFER_SIZE: ::c_int = 14;
pub const _PC_REC_MAX_XFER_SIZE: ::c_int = 15;
pub const _PC_REC_MIN_XFER_SIZE: ::c_int = 16;
pub const _PC_REC_XFER_ALIGN: ::c_int = 17;
pub const _PC_ALLOC_SIZE_MIN: ::c_int = 18;
pub const _PC_SYMLINK_MAX: ::c_int = 19;
pub const _PC_2_SYMLINKS: ::c_int = 20;

pub const _SC_ARG_MAX: ::c_int = 0;
pub const _SC_CHILD_MAX: ::c_int = 1;
pub const _SC_CLK_TCK: ::c_int = 2;
pub const _SC_NGROUPS_MAX: ::c_int = 3;
pub const _SC_OPEN_MAX: ::c_int = 4;
pub const _SC_STREAM_MAX: ::c_int = 5;
pub const _SC_TZNAME_MAX: ::c_int = 6;
pub const _SC_JOB_CONTROL: ::c_int = 7;
pub const _SC_SAVED_IDS: ::c_int = 8;
pub const _SC_REALTIME_SIGNALS: ::c_int = 9;
pub const _SC_PRIORITY_SCHEDULING: ::c_int = 10;
pub const _SC_TIMERS: ::c_int = 11;
pub const _SC_ASYNCHRONOUS_IO: ::c_int = 12;
pub const _SC_PRIORITIZED_IO: ::c_int = 13;
pub const _SC_SYNCHRONIZED_IO: ::c_int = 14;
pub const _SC_FSYNC: ::c_int = 15;
pub const _SC_MAPPED_FILES: ::c_int = 16;
pub const _SC_MEMLOCK: ::c_int = 17;
pub const _SC_MEMLOCK_RANGE: ::c_int = 18;
pub const _SC_MEMORY_PROTECTION: ::c_int = 19;
pub const _SC_MESSAGE_PASSING: ::c_int = 20;
pub const _SC_SEMAPHORES: ::c_int = 21;
pub const _SC_SHARED_MEMORY_OBJECTS: ::c_int = 22;
pub const _SC_AIO_LISTIO_MAX: ::c_int = 23;
pub const _SC_AIO_MAX: ::c_int = 24;
pub const _SC_AIO_PRIO_DELTA_MAX: ::c_int = 25;
pub const _SC_DELAYTIMER_MAX: ::c_int = 26;
pub const _SC_MQ_OPEN_MAX: ::c_int = 27;
pub const _SC_MQ_PRIO_MAX: ::c_int = 28;
pub const _SC_VERSION: ::c_int = 29;
pub const _SC_PAGESIZE: ::c_int = 30;
pub const _SC_PAGE_SIZE: ::c_int = _SC_PAGESIZE;
pub const _SC_RTSIG_MAX: ::c_int = 31;
pub const _SC_SEM_NSEMS_MAX: ::c_int = 32;
pub const _SC_SEM_VALUE_MAX: ::c_int = 33;
pub const _SC_SIGQUEUE_MAX: ::c_int = 34;
pub const _SC_TIMER_MAX: ::c_int = 35;
pub const _SC_BC_BASE_MAX: ::c_int = 36;
pub const _SC_BC_DIM_MAX: ::c_int = 37;
pub const _SC_BC_SCALE_MAX: ::c_int = 38;
pub const _SC_BC_STRING_MAX: ::c_int = 39;
pub const _SC_COLL_WEIGHTS_MAX: ::c_int = 40;
pub const _SC_EXPR_NEST_MAX: ::c_int = 42;
pub const _SC_LINE_MAX: ::c_int = 43;
pub const _SC_RE_DUP_MAX: ::c_int = 44;
pub const _SC_2_VERSION: ::c_int = 46;
pub const _SC_2_C_BIND: ::c_int = 47;
pub const _SC_2_C_DEV: ::c_int = 48;
pub const _SC_2_FORT_DEV: ::c_int = 49;
pub const _SC_2_FORT_RUN: ::c_int = 50;
pub const _SC_2_SW_DEV: ::c_int = 51;
pub const _SC_2_LOCALEDEF: ::c_int = 52;
pub const _SC_UIO_MAXIOV: ::c_int = 60;
pub const _SC_IOV_MAX: ::c_int = 60;
pub const _SC_THREADS: ::c_int = 67;
pub const _SC_THREAD_SAFE_FUNCTIONS: ::c_int = 68;
pub const _SC_GETGR_R_SIZE_MAX: ::c_int = 69;
pub const _SC_GETPW_R_SIZE_MAX: ::c_int = 70;
pub const _SC_LOGIN_NAME_MAX: ::c_int = 71;
pub const _SC_TTY_NAME_MAX: ::c_int = 72;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: ::c_int = 73;
pub const _SC_THREAD_KEYS_MAX: ::c_int = 74;
pub const _SC_THREAD_STACK_MIN: ::c_int = 75;
pub const _SC_THREAD_THREADS_MAX: ::c_int = 76;
pub const _SC_THREAD_ATTR_STACKADDR: ::c_int = 77;
pub const _SC_THREAD_ATTR_STACKSIZE: ::c_int = 78;
pub const _SC_THREAD_PRIORITY_SCHEDULING: ::c_int = 79;
pub const _SC_THREAD_PRIO_INHERIT: ::c_int = 80;
pub const _SC_THREAD_PRIO_PROTECT: ::c_int = 81;
pub const _SC_THREAD_PROCESS_SHARED: ::c_int = 82;
pub const _SC_NPROCESSORS_CONF: ::c_int = 83;
pub const _SC_NPROCESSORS_ONLN: ::c_int = 84;
pub const _SC_PHYS_PAGES: ::c_int = 85;
pub const _SC_AVPHYS_PAGES: ::c_int = 86;
pub const _SC_ATEXIT_MAX: ::c_int = 87;
pub const _SC_PASS_MAX: ::c_int = 88;
pub const _SC_XOPEN_VERSION: ::c_int = 89;
pub const _SC_XOPEN_XCU_VERSION: ::c_int = 90;
pub const _SC_XOPEN_UNIX: ::c_int = 91;
pub const _SC_XOPEN_CRYPT: ::c_int = 92;
pub const _SC_XOPEN_ENH_I18N: ::c_int = 93;
pub const _SC_XOPEN_SHM: ::c_int = 94;
pub const _SC_2_CHAR_TERM: ::c_int = 95;
pub const _SC_2_UPE: ::c_int = 97;
pub const _SC_XOPEN_XPG2: ::c_int = 98;
pub const _SC_XOPEN_XPG3: ::c_int = 99;
pub const _SC_XOPEN_XPG4: ::c_int = 100;
pub const _SC_NZERO: ::c_int = 109;
pub const _SC_XBS5_ILP32_OFF32: ::c_int = 125;
pub const _SC_XBS5_ILP32_OFFBIG: ::c_int = 126;
pub const _SC_XBS5_LP64_OFF64: ::c_int = 127;
pub const _SC_XBS5_LPBIG_OFFBIG: ::c_int = 128;
pub const _SC_XOPEN_LEGACY: ::c_int = 129;
pub const _SC_XOPEN_REALTIME: ::c_int = 130;
pub const _SC_XOPEN_REALTIME_THREADS: ::c_int = 131;
pub const _SC_ADVISORY_INFO: ::c_int = 132;
pub const _SC_BARRIERS: ::c_int = 133;
pub const _SC_CLOCK_SELECTION: ::c_int = 137;
pub const _SC_CPUTIME: ::c_int = 138;
pub const _SC_THREAD_CPUTIME: ::c_int = 139;
pub const _SC_MONOTONIC_CLOCK: ::c_int = 149;
pub const _SC_READER_WRITER_LOCKS: ::c_int = 153;
pub const _SC_SPIN_LOCKS: ::c_int = 154;
pub const _SC_REGEXP: ::c_int = 155;
pub const _SC_SHELL: ::c_int = 157;
pub const _SC_SPAWN: ::c_int = 159;
pub const _SC_SPORADIC_SERVER: ::c_int = 160;
pub const _SC_THREAD_SPORADIC_SERVER: ::c_int = 161;
pub const _SC_TIMEOUTS: ::c_int = 164;
pub const _SC_TYPED_MEMORY_OBJECTS: ::c_int = 165;
pub const _SC_2_PBS: ::c_int = 168;
pub const _SC_2_PBS_ACCOUNTING: ::c_int = 169;
pub const _SC_2_PBS_LOCATE: ::c_int = 170;
pub const _SC_2_PBS_MESSAGE: ::c_int = 171;
pub const _SC_2_PBS_TRACK: ::c_int = 172;
pub const _SC_SYMLOOP_MAX: ::c_int = 173;
pub const _SC_STREAMS: ::c_int = 174;
pub const _SC_2_PBS_CHECKPOINT: ::c_int = 175;
pub const _SC_V6_ILP32_OFF32: ::c_int = 176;
pub const _SC_V6_ILP32_OFFBIG: ::c_int = 177;
pub const _SC_V6_LP64_OFF64: ::c_int = 178;
pub const _SC_V6_LPBIG_OFFBIG: ::c_int = 179;
pub const _SC_HOST_NAME_MAX: ::c_int = 180;
pub const _SC_TRACE: ::c_int = 181;
pub const _SC_TRACE_EVENT_FILTER: ::c_int = 182;
pub const _SC_TRACE_INHERIT: ::c_int = 183;
pub const _SC_TRACE_LOG: ::c_int = 184;
pub const _SC_IPV6: ::c_int = 235;
pub const _SC_RAW_SOCKETS: ::c_int = 236;
pub const _SC_V7_ILP32_OFF32: ::c_int = 237;
pub const _SC_V7_ILP32_OFFBIG: ::c_int = 238;
pub const _SC_V7_LP64_OFF64: ::c_int = 239;
pub const _SC_V7_LPBIG_OFFBIG: ::c_int = 240;
pub const _SC_SS_REPL_MAX: ::c_int = 241;
pub const _SC_TRACE_EVENT_NAME_MAX: ::c_int = 242;
pub const _SC_TRACE_NAME_MAX: ::c_int = 243;
pub const _SC_TRACE_SYS_MAX: ::c_int = 244;
pub const _SC_TRACE_USER_EVENT_MAX: ::c_int = 245;
pub const _SC_XOPEN_STREAMS: ::c_int = 246;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: ::c_int = 247;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: ::c_int = 248;

pub const RLIM_SAVED_MAX: ::rlim_t = RLIM_INFINITY;
pub const RLIM_SAVED_CUR: ::rlim_t = RLIM_INFINITY;

pub const GLOB_ERR: ::c_int = 1 << 0;
pub const GLOB_MARK: ::c_int = 1 << 1;
pub const GLOB_NOSORT: ::c_int = 1 << 2;
pub const GLOB_DOOFFS: ::c_int = 1 << 3;
pub const GLOB_NOCHECK: ::c_int = 1 << 4;
pub const GLOB_APPEND: ::c_int = 1 << 5;
pub const GLOB_NOESCAPE: ::c_int = 1 << 6;

pub const GLOB_NOSPACE: ::c_int = 1;
pub const GLOB_ABORTED: ::c_int = 2;
pub const GLOB_NOMATCH: ::c_int = 3;

pub const POSIX_MADV_NORMAL: ::c_int = 0;
pub const POSIX_MADV_RANDOM: ::c_int = 1;
pub const POSIX_MADV_SEQUENTIAL: ::c_int = 2;
pub const POSIX_MADV_WILLNEED: ::c_int = 3;

pub const S_IEXEC: mode_t = 64;
pub const S_IWRITE: mode_t = 128;
pub const S_IREAD: mode_t = 256;

pub const F_LOCK: ::c_int = 1;
pub const F_TEST: ::c_int = 3;
pub const F_TLOCK: ::c_int = 2;
pub const F_ULOCK: ::c_int = 0;

pub const IFF_LOWER_UP: ::c_int = 0x10000;
pub const IFF_DORMANT: ::c_int = 0x20000;
pub const IFF_ECHO: ::c_int = 0x40000;

// linux/if_tun.h
pub const IFF_TUN: ::c_short = 0x0001;
pub const IFF_TAP: ::c_short = 0x0002;
pub const IFF_NO_PI: ::c_short = 0x1000;
// Read queue size
pub const TUN_READQ_SIZE: ::c_short = 500;
// TUN device type flags: deprecated. Use IFF_TUN/IFF_TAP instead.
pub const TUN_TUN_DEV: ::c_short   = ::IFF_TUN;
pub const TUN_TAP_DEV: ::c_short   = ::IFF_TAP;
pub const TUN_TYPE_MASK: ::c_short = 0x000f;
// This flag has no real effect
pub const IFF_ONE_QUEUE: ::c_short    = 0x2000;
pub const IFF_VNET_HDR: ::c_short     = 0x4000;
pub const IFF_TUN_EXCL: ::c_short     = 0x8000;
pub const IFF_MULTI_QUEUE: ::c_short  = 0x0100;
pub const IFF_ATTACH_QUEUE: ::c_short = 0x0200;
pub const IFF_DETACH_QUEUE: ::c_short = 0x0400;
// read-only flag
pub const IFF_PERSIST: ::c_short  = 0x0800;
pub const IFF_NOFILTER: ::c_short = 0x1000;

pub const ST_RDONLY: ::c_ulong = 1;
pub const ST_NOSUID: ::c_ulong = 2;
pub const ST_NODEV: ::c_ulong = 4;
pub const ST_NOEXEC: ::c_ulong = 8;
pub const ST_SYNCHRONOUS: ::c_ulong = 16;
pub const ST_MANDLOCK: ::c_ulong = 64;
pub const ST_WRITE: ::c_ulong = 128;
pub const ST_APPEND: ::c_ulong = 256;
pub const ST_IMMUTABLE: ::c_ulong = 512;
pub const ST_NOATIME: ::c_ulong = 1024;
pub const ST_NODIRATIME: ::c_ulong = 2048;

pub const RTLD_NEXT: *mut ::c_void = -1i64 as *mut ::c_void;
pub const RTLD_DEFAULT: *mut ::c_void = 0i64 as *mut ::c_void;
pub const RTLD_NODELETE: ::c_int = 0x1000;
pub const RTLD_NOW: ::c_int = 0x2;

pub const TCP_MD5SIG: ::c_int = 14;

pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    __align: [],
    size: [0; __SIZEOF_PTHREAD_MUTEX_T],
};
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    __align: [],
    size: [0; __SIZEOF_PTHREAD_COND_T],
};
pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
    __align: [],
    size: [0; __SIZEOF_PTHREAD_RWLOCK_T],
};
pub const PTHREAD_MUTEX_NORMAL: ::c_int = 0;
pub const PTHREAD_MUTEX_RECURSIVE: ::c_int = 1;
pub const PTHREAD_MUTEX_ERRORCHECK: ::c_int = 2;
pub const PTHREAD_MUTEX_DEFAULT: ::c_int = PTHREAD_MUTEX_NORMAL;
pub const PTHREAD_PROCESS_PRIVATE: ::c_int = 0;
pub const PTHREAD_PROCESS_SHARED: ::c_int = 1;
pub const __SIZEOF_PTHREAD_COND_T: usize = 48;

pub const RENAME_NOREPLACE: ::c_int = 1;
pub const RENAME_EXCHANGE: ::c_int = 2;
pub const RENAME_WHITEOUT: ::c_int = 4;

pub const SCHED_OTHER: ::c_int = 0;
pub const SCHED_FIFO: ::c_int = 1;
pub const SCHED_RR: ::c_int = 2;
pub const SCHED_BATCH: ::c_int = 3;
pub const SCHED_IDLE: ::c_int = 5;

// netinet/in.h
// NOTE: These are in addition to the constants defined in src/unix/mod.rs

// IPPROTO_IP defined in src/unix/mod.rs
/// Hop-by-hop option header
pub const IPPROTO_HOPOPTS: ::c_int = 0;
// IPPROTO_ICMP defined in src/unix/mod.rs
/// group mgmt protocol
pub const IPPROTO_IGMP: ::c_int = 2;
/// for compatibility
pub const IPPROTO_IPIP: ::c_int = 4;
// IPPROTO_TCP defined in src/unix/mod.rs
/// exterior gateway protocol
pub const IPPROTO_EGP: ::c_int = 8;
/// pup
pub const IPPROTO_PUP: ::c_int = 12;
// IPPROTO_UDP defined in src/unix/mod.rs
/// xns idp
pub const IPPROTO_IDP: ::c_int = 22;
/// tp-4 w/ class negotiation
pub const IPPROTO_TP: ::c_int = 29;
/// DCCP
pub const IPPROTO_DCCP: ::c_int = 33;
// IPPROTO_IPV6 defined in src/unix/mod.rs
/// IP6 routing header
pub const IPPROTO_ROUTING: ::c_int = 43;
/// IP6 fragmentation header
pub const IPPROTO_FRAGMENT: ::c_int = 44;
/// resource reservation
pub const IPPROTO_RSVP: ::c_int = 46;
/// General Routing Encap.
pub const IPPROTO_GRE: ::c_int = 47;
/// IP6 Encap Sec. Payload
pub const IPPROTO_ESP: ::c_int = 50;
/// IP6 Auth Header
pub const IPPROTO_AH: ::c_int = 51;
// IPPROTO_ICMPV6 defined in src/unix/mod.rs
/// IP6 no next header
pub const IPPROTO_NONE: ::c_int = 59;
/// IP6 destination option
pub const IPPROTO_DSTOPTS: ::c_int = 60;
pub const IPPROTO_MTP: ::c_int = 92;
pub const IPPROTO_BEETPH: ::c_int = 94;
/// encapsulation header
pub const IPPROTO_ENCAP: ::c_int = 98;
/// Protocol indep. multicast
pub const IPPROTO_PIM: ::c_int = 103;
/// IP Payload Comp. Protocol
pub const IPPROTO_COMP: ::c_int = 108;
/// SCTP
pub const IPPROTO_SCTP: ::c_int = 132;
pub const IPPROTO_MH: ::c_int = 135;
pub const IPPROTO_UDPLITE: ::c_int = 136;
pub const IPPROTO_MPLS: ::c_int = 137;
/// raw IP packet
pub const IPPROTO_RAW: ::c_int = 255;
pub const IPPROTO_MAX: ::c_int = 256;

pub const AF_IB: ::c_int = 27;
pub const AF_MPLS: ::c_int = 28;
pub const AF_NFC: ::c_int = 39;
pub const AF_VSOCK: ::c_int = 40;
pub const PF_IB: ::c_int = AF_IB;
pub const PF_MPLS: ::c_int = AF_MPLS;
pub const PF_NFC: ::c_int = AF_NFC;
pub const PF_VSOCK: ::c_int = AF_VSOCK;

// System V IPC
pub const IPC_PRIVATE: ::key_t = 0;

pub const IPC_CREAT: ::c_int = 0o1000;
pub const IPC_EXCL: ::c_int = 0o2000;
pub const IPC_NOWAIT: ::c_int = 0o4000;

pub const IPC_RMID: ::c_int = 0;
pub const IPC_SET: ::c_int = 1;
pub const IPC_STAT: ::c_int = 2;
pub const IPC_INFO: ::c_int = 3;
pub const MSG_STAT: ::c_int = 11;
pub const MSG_INFO: ::c_int = 12;

pub const MSG_NOERROR: ::c_int = 0o10000;
pub const MSG_EXCEPT: ::c_int = 0o20000;
pub const MSG_COPY: ::c_int = 0o40000;

pub const SHM_R: ::c_int = 0o400;
pub const SHM_W: ::c_int = 0o200;

pub const SHM_RDONLY: ::c_int = 0o10000;
pub const SHM_RND: ::c_int = 0o20000;
pub const SHM_REMAP: ::c_int = 0o40000;
pub const SHM_EXEC: ::c_int = 0o100000;

pub const SHM_LOCK: ::c_int = 11;
pub const SHM_UNLOCK: ::c_int = 12;

pub const SHM_HUGETLB: ::c_int = 0o4000;
pub const SHM_NORESERVE: ::c_int = 0o10000;

pub const EPOLLRDHUP: ::c_int = 0x2000;
pub const EPOLLEXCLUSIVE: ::c_int = 0x10000000;
pub const EPOLLONESHOT: ::c_int = 0x40000000;

pub const QFMT_VFS_OLD: ::c_int = 1;
pub const QFMT_VFS_V0: ::c_int = 2;
pub const QFMT_VFS_V1: ::c_int = 4;

pub const EFD_SEMAPHORE: ::c_int = 0x1;

pub const LOG_NFACILITIES: ::c_int = 24;

pub const SEM_FAILED: *mut ::sem_t = 0 as *mut sem_t;

pub const RB_AUTOBOOT: ::c_int = 0x01234567u32 as i32;
pub const RB_HALT_SYSTEM: ::c_int = 0xcdef0123u32 as i32;
pub const RB_ENABLE_CAD: ::c_int = 0x89abcdefu32 as i32;
pub const RB_DISABLE_CAD: ::c_int = 0x00000000u32 as i32;
pub const RB_POWER_OFF: ::c_int = 0x4321fedcu32 as i32;
pub const RB_SW_SUSPEND: ::c_int = 0xd000fce2u32 as i32;
pub const RB_KEXEC: ::c_int = 0x45584543u32 as i32;

pub const AI_PASSIVE: ::c_int = 0x0001;
pub const AI_CANONNAME: ::c_int = 0x0002;
pub const AI_NUMERICHOST: ::c_int = 0x0004;
pub const AI_V4MAPPED: ::c_int = 0x0008;
pub const AI_ALL: ::c_int = 0x0010;
pub const AI_ADDRCONFIG: ::c_int = 0x0020;

pub const AI_NUMERICSERV: ::c_int = 0x0400;

pub const EAI_BADFLAGS: ::c_int = -1;
pub const EAI_NONAME: ::c_int = -2;
pub const EAI_AGAIN: ::c_int = -3;
pub const EAI_FAIL: ::c_int = -4;
pub const EAI_NODATA: ::c_int = -5;
pub const EAI_FAMILY: ::c_int = -6;
pub const EAI_SOCKTYPE: ::c_int = -7;
pub const EAI_SERVICE: ::c_int = -8;
pub const EAI_MEMORY: ::c_int = -10;
pub const EAI_SYSTEM: ::c_int = -11;
pub const EAI_OVERFLOW: ::c_int = -12;

pub const NI_NUMERICHOST: ::c_int = 1;
pub const NI_NUMERICSERV: ::c_int = 2;
pub const NI_NOFQDN: ::c_int = 4;
pub const NI_NAMEREQD: ::c_int = 8;
pub const NI_DGRAM: ::c_int = 16;

pub const SYNC_FILE_RANGE_WAIT_BEFORE: ::c_uint = 1;
pub const SYNC_FILE_RANGE_WRITE: ::c_uint = 2;
pub const SYNC_FILE_RANGE_WAIT_AFTER: ::c_uint = 4;

pub const AIO_CANCELED: ::c_int = 0;
pub const AIO_NOTCANCELED: ::c_int = 1;
pub const AIO_ALLDONE: ::c_int = 2;
pub const LIO_READ: ::c_int = 0;
pub const LIO_WRITE: ::c_int = 1;
pub const LIO_NOP: ::c_int = 2;
pub const LIO_WAIT: ::c_int = 0;
pub const LIO_NOWAIT: ::c_int = 1;

pub const MREMAP_MAYMOVE: ::c_int = 1;
pub const MREMAP_FIXED: ::c_int = 2;

pub const PR_SET_PDEATHSIG: ::c_int = 1;
pub const PR_GET_PDEATHSIG: ::c_int = 2;

pub const PR_GET_DUMPABLE: ::c_int = 3;
pub const PR_SET_DUMPABLE: ::c_int = 4;

pub const PR_GET_UNALIGN: ::c_int = 5;
pub const PR_SET_UNALIGN: ::c_int = 6;
pub const PR_UNALIGN_NOPRINT: ::c_int = 1;
pub const PR_UNALIGN_SIGBUS: ::c_int = 2;

pub const PR_GET_KEEPCAPS: ::c_int = 7;
pub const PR_SET_KEEPCAPS: ::c_int = 8;

pub const PR_GET_FPEMU: ::c_int = 9;
pub const PR_SET_FPEMU: ::c_int = 10;
pub const PR_FPEMU_NOPRINT: ::c_int = 1;
pub const PR_FPEMU_SIGFPE: ::c_int = 2;

pub const PR_GET_FPEXC: ::c_int = 11;
pub const PR_SET_FPEXC: ::c_int = 12;
pub const PR_FP_EXC_SW_ENABLE: ::c_int = 0x80;
pub const PR_FP_EXC_DIV: ::c_int = 0x010000;
pub const PR_FP_EXC_OVF: ::c_int = 0x020000;
pub const PR_FP_EXC_UND: ::c_int = 0x040000;
pub const PR_FP_EXC_RES: ::c_int = 0x080000;
pub const PR_FP_EXC_INV: ::c_int = 0x100000;
pub const PR_FP_EXC_DISABLED: ::c_int = 0;
pub const PR_FP_EXC_NONRECOV: ::c_int = 1;
pub const PR_FP_EXC_ASYNC: ::c_int = 2;
pub const PR_FP_EXC_PRECISE: ::c_int = 3;

pub const PR_GET_TIMING: ::c_int = 13;
pub const PR_SET_TIMING: ::c_int = 14;
pub const PR_TIMING_STATISTICAL: ::c_int = 0;
pub const PR_TIMING_TIMESTAMP: ::c_int = 1;

pub const PR_SET_NAME: ::c_int = 15;
pub const PR_GET_NAME: ::c_int = 16;

pub const PR_GET_ENDIAN: ::c_int = 19;
pub const PR_SET_ENDIAN: ::c_int = 20;
pub const PR_ENDIAN_BIG: ::c_int = 0;
pub const PR_ENDIAN_LITTLE: ::c_int = 1;
pub const PR_ENDIAN_PPC_LITTLE: ::c_int = 2;

pub const PR_GET_SECCOMP: ::c_int = 21;
pub const PR_SET_SECCOMP: ::c_int = 22;

pub const PR_CAPBSET_READ: ::c_int = 23;
pub const PR_CAPBSET_DROP: ::c_int = 24;

pub const PR_GET_TSC: ::c_int = 25;
pub const PR_SET_TSC: ::c_int = 26;
pub const PR_TSC_ENABLE: ::c_int = 1;
pub const PR_TSC_SIGSEGV: ::c_int = 2;

pub const PR_GET_SECUREBITS: ::c_int = 27;
pub const PR_SET_SECUREBITS: ::c_int = 28;

pub const PR_SET_TIMERSLACK: ::c_int = 29;
pub const PR_GET_TIMERSLACK: ::c_int = 30;

pub const PR_TASK_PERF_EVENTS_DISABLE: ::c_int = 31;
pub const PR_TASK_PERF_EVENTS_ENABLE: ::c_int = 32;

pub const PR_MCE_KILL: ::c_int = 33;
pub const PR_MCE_KILL_CLEAR: ::c_int = 0;
pub const PR_MCE_KILL_SET: ::c_int = 1;

pub const PR_MCE_KILL_LATE: ::c_int = 0;
pub const PR_MCE_KILL_EARLY: ::c_int = 1;
pub const PR_MCE_KILL_DEFAULT: ::c_int = 2;

pub const PR_MCE_KILL_GET: ::c_int = 34;

pub const PR_SET_MM: ::c_int = 35;
pub const PR_SET_MM_START_CODE: ::c_int = 1;
pub const PR_SET_MM_END_CODE: ::c_int = 2;
pub const PR_SET_MM_START_DATA: ::c_int = 3;
pub const PR_SET_MM_END_DATA: ::c_int = 4;
pub const PR_SET_MM_START_STACK: ::c_int = 5;
pub const PR_SET_MM_START_BRK: ::c_int = 6;
pub const PR_SET_MM_BRK: ::c_int = 7;
pub const PR_SET_MM_ARG_START: ::c_int = 8;
pub const PR_SET_MM_ARG_END: ::c_int = 9;
pub const PR_SET_MM_ENV_START: ::c_int = 10;
pub const PR_SET_MM_ENV_END: ::c_int = 11;
pub const PR_SET_MM_AUXV: ::c_int = 12;
pub const PR_SET_MM_EXE_FILE: ::c_int = 13;
pub const PR_SET_MM_MAP: ::c_int = 14;
pub const PR_SET_MM_MAP_SIZE: ::c_int = 15;

pub const PR_SET_PTRACER: ::c_int = 0x59616d61;

pub const PR_SET_CHILD_SUBREAPER: ::c_int = 36;
pub const PR_GET_CHILD_SUBREAPER: ::c_int = 37;

pub const PR_SET_NO_NEW_PRIVS: ::c_int = 38;
pub const PR_GET_NO_NEW_PRIVS: ::c_int = 39;

pub const PR_GET_TID_ADDRESS: ::c_int = 40;

pub const PR_SET_THP_DISABLE: ::c_int = 41;
pub const PR_GET_THP_DISABLE: ::c_int = 42;

pub const PR_MPX_ENABLE_MANAGEMENT: ::c_int = 43;
pub const PR_MPX_DISABLE_MANAGEMENT: ::c_int = 44;

pub const PR_SET_FP_MODE: ::c_int = 45;
pub const PR_GET_FP_MODE: ::c_int = 46;
pub const PR_FP_MODE_FR: ::c_int = 1 << 0;
pub const PR_FP_MODE_FRE: ::c_int = 1 << 1;

pub const PR_CAP_AMBIENT: ::c_int = 47;
pub const PR_CAP_AMBIENT_IS_SET: ::c_int = 1;
pub const PR_CAP_AMBIENT_RAISE: ::c_int = 2;
pub const PR_CAP_AMBIENT_LOWER: ::c_int = 3;
pub const PR_CAP_AMBIENT_CLEAR_ALL: ::c_int = 4;

pub const GRND_NONBLOCK: ::c_uint = 0x0001;
pub const GRND_RANDOM: ::c_uint = 0x0002;

pub const SECCOMP_MODE_DISABLED: ::c_uint = 0;
pub const SECCOMP_MODE_STRICT: ::c_uint = 1;
pub const SECCOMP_MODE_FILTER: ::c_uint = 2;

pub const ITIMER_REAL: ::c_int = 0;
pub const ITIMER_VIRTUAL: ::c_int = 1;
pub const ITIMER_PROF: ::c_int = 2;

pub const TFD_CLOEXEC: ::c_int = O_CLOEXEC;
pub const TFD_NONBLOCK: ::c_int = O_NONBLOCK;
pub const TFD_TIMER_ABSTIME: ::c_int = 1;

pub const XATTR_CREATE: ::c_int = 0x1;
pub const XATTR_REPLACE: ::c_int = 0x2;

pub const _POSIX_VDISABLE: ::cc_t = 0;

pub const FALLOC_FL_KEEP_SIZE: ::c_int = 0x01;
pub const FALLOC_FL_PUNCH_HOLE: ::c_int = 0x02;
pub const FALLOC_FL_COLLAPSE_RANGE: ::c_int = 0x08;
pub const FALLOC_FL_ZERO_RANGE: ::c_int = 0x10;
pub const FALLOC_FL_INSERT_RANGE: ::c_int = 0x20;
pub const FALLOC_FL_UNSHARE_RANGE: ::c_int = 0x40;

// On Linux, libc doesn't define this constant, libattr does instead.
// We still define it for Linux as it's defined by libc on other platforms,
// and it's mentioned in the man pages for getxattr and setxattr.
pub const ENOATTR: ::c_int = ::ENODATA;

pub const SO_ORIGINAL_DST: ::c_int = 80;
pub const IUTF8: ::tcflag_t = 0x00004000;
pub const CMSPAR: ::tcflag_t = 0o10000000000;

pub const MFD_CLOEXEC: ::c_uint = 0x0001;
pub const MFD_ALLOW_SEALING: ::c_uint = 0x0002;

// these are used in the p_type field of Elf32_Phdr and Elf64_Phdr, which has
// the type Elf32Word and Elf64Word respectively. Luckily, both of those are u32
// so we can use that type here to avoid having to cast.
pub const PT_NULL: u32 = 0;
pub const PT_LOAD: u32 = 1;
pub const PT_DYNAMIC: u32 = 2;
pub const PT_INTERP: u32 = 3;
pub const PT_NOTE: u32 = 4;
pub const PT_SHLIB: u32 = 5;
pub const PT_PHDR: u32 = 6;
pub const PT_TLS: u32 = 7;
pub const PT_NUM: u32 = 8;
pub const PT_LOOS: u32 = 0x60000000;
pub const PT_GNU_EH_FRAME: u32 = 0x6474e550;
pub const PT_GNU_STACK: u32 = 0x6474e551;
pub const PT_GNU_RELRO: u32 = 0x6474e552;

// linux/if_ether.h
pub const ETH_ALEN: ::c_int = 6;
pub const ETH_HLEN: ::c_int = 14;
pub const ETH_ZLEN: ::c_int = 60;
pub const ETH_DATA_LEN: ::c_int = 1500;
pub const ETH_FRAME_LEN: ::c_int = 1514;
pub const ETH_FCS_LEN: ::c_int = 4;

// These are the defined Ethernet Protocol ID's.
pub const ETH_P_LOOP: ::c_int = 0x0060;
pub const ETH_P_PUP: ::c_int = 0x0200;
pub const ETH_P_PUPAT: ::c_int = 0x0201;
pub const ETH_P_IP: ::c_int = 0x0800;
pub const ETH_P_X25: ::c_int = 0x0805;
pub const ETH_P_ARP: ::c_int = 0x0806;
pub const ETH_P_BPQ: ::c_int = 0x08FF;
pub const ETH_P_IEEEPUP: ::c_int = 0x0a00;
pub const ETH_P_IEEEPUPAT: ::c_int = 0x0a01;
pub const ETH_P_BATMAN: ::c_int = 0x4305;
pub const ETH_P_DEC: ::c_int = 0x6000;
pub const ETH_P_DNA_DL: ::c_int = 0x6001;
pub const ETH_P_DNA_RC: ::c_int = 0x6002;
pub const ETH_P_DNA_RT: ::c_int = 0x6003;
pub const ETH_P_LAT: ::c_int = 0x6004;
pub const ETH_P_DIAG: ::c_int = 0x6005;
pub const ETH_P_CUST: ::c_int = 0x6006;
pub const ETH_P_SCA: ::c_int = 0x6007;
pub const ETH_P_TEB: ::c_int = 0x6558;
pub const ETH_P_RARP: ::c_int = 0x8035;
pub const ETH_P_ATALK: ::c_int = 0x809B;
pub const ETH_P_AARP: ::c_int = 0x80F3;
pub const ETH_P_8021Q: ::c_int = 0x8100;
pub const ETH_P_IPX: ::c_int = 0x8137;
pub const ETH_P_IPV6: ::c_int = 0x86DD;
pub const ETH_P_PAUSE: ::c_int = 0x8808;
pub const ETH_P_SLOW: ::c_int = 0x8809;
pub const ETH_P_WCCP: ::c_int = 0x883E;
pub const ETH_P_MPLS_UC: ::c_int = 0x8847;
pub const ETH_P_MPLS_MC: ::c_int = 0x8848;
pub const ETH_P_ATMMPOA: ::c_int = 0x884c;
pub const ETH_P_PPP_DISC: ::c_int = 0x8863;
pub const ETH_P_PPP_SES: ::c_int = 0x8864;
pub const ETH_P_LINK_CTL: ::c_int = 0x886c;
pub const ETH_P_ATMFATE: ::c_int = 0x8884;
pub const ETH_P_PAE: ::c_int = 0x888E;
pub const ETH_P_AOE: ::c_int = 0x88A2;
pub const ETH_P_8021AD: ::c_int = 0x88A8;
pub const ETH_P_802_EX1: ::c_int = 0x88B5;
pub const ETH_P_TIPC: ::c_int = 0x88CA;
pub const ETH_P_MACSEC: ::c_int = 0x88E5;
pub const ETH_P_8021AH: ::c_int = 0x88E7;
pub const ETH_P_MVRP: ::c_int = 0x88F5;
pub const ETH_P_1588: ::c_int = 0x88F7;
pub const ETH_P_PRP: ::c_int = 0x88FB;
pub const ETH_P_FCOE: ::c_int = 0x8906;
pub const ETH_P_TDLS: ::c_int = 0x890D;
pub const ETH_P_FIP: ::c_int = 0x8914;
pub const ETH_P_80221: ::c_int = 0x8917;
pub const ETH_P_LOOPBACK: ::c_int = 0x9000;
pub const ETH_P_QINQ1: ::c_int = 0x9100;
pub const ETH_P_QINQ2: ::c_int = 0x9200;
pub const ETH_P_QINQ3: ::c_int = 0x9300;
pub const ETH_P_EDSA: ::c_int = 0xDADA;
pub const ETH_P_AF_IUCV: ::c_int = 0xFBFB;

pub const ETH_P_802_3_MIN: ::c_int = 0x0600;

// Non DIX types. Won't clash for 1500 types.
pub const ETH_P_802_3: ::c_int = 0x0001;
pub const ETH_P_AX25: ::c_int = 0x0002;
pub const ETH_P_ALL: ::c_int = 0x0003;
pub const ETH_P_802_2: ::c_int = 0x0004;
pub const ETH_P_SNAP: ::c_int = 0x0005;
pub const ETH_P_DDCMP: ::c_int = 0x0006;
pub const ETH_P_WAN_PPP: ::c_int = 0x0007;
pub const ETH_P_PPP_MP: ::c_int = 0x0008;
pub const ETH_P_LOCALTALK: ::c_int = 0x0009;
pub const ETH_P_CANFD: ::c_int = 0x000D;
pub const ETH_P_PPPTALK: ::c_int = 0x0010;
pub const ETH_P_TR_802_2: ::c_int = 0x0011;
pub const ETH_P_MOBITEX: ::c_int = 0x0015;
pub const ETH_P_CONTROL: ::c_int = 0x0016;
pub const ETH_P_IRDA: ::c_int = 0x0017;
pub const ETH_P_ECONET: ::c_int = 0x0018;
pub const ETH_P_HDLC: ::c_int = 0x0019;
pub const ETH_P_ARCNET: ::c_int = 0x001A;
pub const ETH_P_DSA: ::c_int = 0x001B;
pub const ETH_P_TRAILER: ::c_int = 0x001C;
pub const ETH_P_PHONET: ::c_int = 0x00F5;
pub const ETH_P_IEEE802154: ::c_int = 0x00F6;
pub const ETH_P_CAIF: ::c_int = 0x00F7;

pub const POSIX_SPAWN_RESETIDS: ::c_int = 0x01;
pub const POSIX_SPAWN_SETPGROUP: ::c_int = 0x02;
pub const POSIX_SPAWN_SETSIGDEF: ::c_int = 0x04;
pub const POSIX_SPAWN_SETSIGMASK: ::c_int = 0x08;
pub const POSIX_SPAWN_SETSCHEDPARAM: ::c_int = 0x10;
pub const POSIX_SPAWN_SETSCHEDULER: ::c_int = 0x20;

pub const NLMSG_NOOP: ::c_int = 0x1;
pub const NLMSG_ERROR: ::c_int = 0x2;
pub const NLMSG_DONE: ::c_int = 0x3;
pub const NLMSG_OVERRUN: ::c_int = 0x4;
pub const NLMSG_MIN_TYPE: ::c_int = 0x10;

pub const GENL_NAMSIZ: ::c_int = 16;

pub const GENL_MIN_ID: ::c_int = NLMSG_MIN_TYPE;
pub const GENL_MAX_ID: ::c_int = 1023;

pub const GENL_ADMIN_PERM: ::c_int = 0x01;
pub const GENL_CMD_CAP_DO: ::c_int = 0x02;
pub const GENL_CMD_CAP_DUMP: ::c_int = 0x04;
pub const GENL_CMD_CAP_HASPOL: ::c_int = 0x08;

pub const GENL_ID_CTRL: ::c_int = NLMSG_MIN_TYPE;

pub const CTRL_CMD_UNSPEC: ::c_int = 0;
pub const CTRL_CMD_NEWFAMILY: ::c_int = 1;
pub const CTRL_CMD_DELFAMILY: ::c_int = 2;
pub const CTRL_CMD_GETFAMILY: ::c_int = 3;
pub const CTRL_CMD_NEWOPS: ::c_int = 4;
pub const CTRL_CMD_DELOPS: ::c_int = 5;
pub const CTRL_CMD_GETOPS: ::c_int = 6;
pub const CTRL_CMD_NEWMCAST_GRP: ::c_int = 7;
pub const CTRL_CMD_DELMCAST_GRP: ::c_int = 8;
pub const CTRL_CMD_GETMCAST_GRP: ::c_int = 9;

pub const CTRL_ATTR_UNSPEC: ::c_int = 0;
pub const CTRL_ATTR_FAMILY_ID: ::c_int = 1;
pub const CTRL_ATTR_FAMILY_NAME: ::c_int = 2;
pub const CTRL_ATTR_VERSION: ::c_int = 3;
pub const CTRL_ATTR_HDRSIZE: ::c_int = 4;
pub const CTRL_ATTR_MAXATTR: ::c_int = 5;
pub const CTRL_ATTR_OPS: ::c_int = 6;
pub const CTRL_ATTR_MCAST_GROUPS: ::c_int = 7;

pub const CTRL_ATTR_OP_UNSPEC: ::c_int = 0;
pub const CTRL_ATTR_OP_ID: ::c_int = 1;
pub const CTRL_ATTR_OP_FLAGS: ::c_int = 2;

pub const CTRL_ATTR_MCAST_GRP_UNSPEC: ::c_int = 0;
pub const CTRL_ATTR_MCAST_GRP_NAME: ::c_int = 1;
pub const CTRL_ATTR_MCAST_GRP_ID: ::c_int = 2;

// linux/netfilter.h
pub const NF_DROP: ::c_int = 0;
pub const NF_ACCEPT: ::c_int =  1;
pub const NF_STOLEN: ::c_int =  2;
pub const NF_QUEUE: ::c_int =  3;
pub const NF_REPEAT: ::c_int =  4;
pub const NF_STOP: ::c_int =  5;
pub const NF_MAX_VERDICT: ::c_int = NF_STOP;

pub const NF_VERDICT_MASK: ::c_int = 0x000000ff;
pub const NF_VERDICT_FLAG_QUEUE_BYPASS: ::c_int = 0x00008000;

pub const NF_VERDICT_QMASK: ::c_int = 0xffff0000;
pub const NF_VERDICT_QBITS: ::c_int = 16;

pub const NF_VERDICT_BITS: ::c_int = 16;

pub const NF_INET_PRE_ROUTING: ::c_int = 0;
pub const NF_INET_LOCAL_IN: ::c_int = 1;
pub const NF_INET_FORWARD: ::c_int = 2;
pub const NF_INET_LOCAL_OUT: ::c_int = 3;
pub const NF_INET_POST_ROUTING: ::c_int = 4;
pub const NF_INET_NUMHOOKS: ::c_int = 5;

// Some NFPROTO are not compatible with musl and are defined in submodules.
pub const NFPROTO_UNSPEC: ::c_int = 0;
pub const NFPROTO_IPV4: ::c_int = 2;
pub const NFPROTO_ARP: ::c_int = 3;
pub const NFPROTO_BRIDGE: ::c_int = 7;
pub const NFPROTO_IPV6: ::c_int = 10;
pub const NFPROTO_DECNET: ::c_int = 12;
pub const NFPROTO_NUMPROTO: ::c_int = 13;

// linux/netfilter_ipv4.h
pub const NF_IP_PRE_ROUTING: ::c_int = 0;
pub const NF_IP_LOCAL_IN: ::c_int = 1;
pub const NF_IP_FORWARD: ::c_int = 2;
pub const NF_IP_LOCAL_OUT: ::c_int = 3;
pub const NF_IP_POST_ROUTING: ::c_int = 4;
pub const NF_IP_NUMHOOKS: ::c_int = 5;

pub const NF_IP_PRI_FIRST: ::c_int = ::INT_MIN;
pub const NF_IP_PRI_CONNTRACK_DEFRAG: ::c_int = -400;
pub const NF_IP_PRI_RAW: ::c_int = -300;
pub const NF_IP_PRI_SELINUX_FIRST: ::c_int = -225;
pub const NF_IP_PRI_CONNTRACK: ::c_int = -200;
pub const NF_IP_PRI_MANGLE: ::c_int = -150;
pub const NF_IP_PRI_NAT_DST: ::c_int = -100;
pub const NF_IP_PRI_FILTER: ::c_int = 0;
pub const NF_IP_PRI_SECURITY: ::c_int = 50;
pub const NF_IP_PRI_NAT_SRC: ::c_int = 100;
pub const NF_IP_PRI_SELINUX_LAST: ::c_int = 225;
pub const NF_IP_PRI_CONNTRACK_HELPER: ::c_int = 300;
pub const NF_IP_PRI_CONNTRACK_CONFIRM: ::c_int = ::INT_MAX;
pub const NF_IP_PRI_LAST: ::c_int = ::INT_MAX;

// linux/netfilter_ipv6.h
pub const NF_IP6_PRE_ROUTING: ::c_int = 0;
pub const NF_IP6_LOCAL_IN: ::c_int = 1;
pub const NF_IP6_FORWARD: ::c_int = 2;
pub const NF_IP6_LOCAL_OUT: ::c_int = 3;
pub const NF_IP6_POST_ROUTING: ::c_int = 4;
pub const NF_IP6_NUMHOOKS: ::c_int = 5;

pub const NF_IP6_PRI_FIRST: ::c_int = ::INT_MIN;
pub const NF_IP6_PRI_CONNTRACK_DEFRAG: ::c_int = -400;
pub const NF_IP6_PRI_RAW: ::c_int = -300;
pub const NF_IP6_PRI_SELINUX_FIRST: ::c_int = -225;
pub const NF_IP6_PRI_CONNTRACK: ::c_int = -200;
pub const NF_IP6_PRI_MANGLE: ::c_int = -150;
pub const NF_IP6_PRI_NAT_DST: ::c_int = -100;
pub const NF_IP6_PRI_FILTER: ::c_int = 0;
pub const NF_IP6_PRI_SECURITY: ::c_int = 50;
pub const NF_IP6_PRI_NAT_SRC: ::c_int = 100;
pub const NF_IP6_PRI_SELINUX_LAST: ::c_int = 225;
pub const NF_IP6_PRI_CONNTRACK_HELPER: ::c_int = 300;
pub const NF_IP6_PRI_LAST: ::c_int = ::INT_MAX;

f! {
    pub fn CPU_ZERO(cpuset: &mut cpu_set_t) -> () {
        for slot in cpuset.bits.iter_mut() {
            *slot = 0;
        }
    }

    pub fn CPU_SET(cpu: usize, cpuset: &mut cpu_set_t) -> () {
        let size_in_bits = 8 * mem::size_of_val(&cpuset.bits[0]); // 32, 64 etc
        let (idx, offset) = (cpu / size_in_bits, cpu % size_in_bits);
        cpuset.bits[idx] |= 1 << offset;
        ()
    }

    pub fn CPU_CLR(cpu: usize, cpuset: &mut cpu_set_t) -> () {
        let size_in_bits = 8 * mem::size_of_val(&cpuset.bits[0]); // 32, 64 etc
        let (idx, offset) = (cpu / size_in_bits, cpu % size_in_bits);
        cpuset.bits[idx] &= !(1 << offset);
        ()
    }

    pub fn CPU_ISSET(cpu: usize, cpuset: &cpu_set_t) -> bool {
        let size_in_bits = 8 * mem::size_of_val(&cpuset.bits[0]);
        let (idx, offset) = (cpu / size_in_bits, cpu % size_in_bits);
        0 != (cpuset.bits[idx] & (1 << offset))
    }

    pub fn CPU_EQUAL(set1: &cpu_set_t, set2: &cpu_set_t) -> bool {
        set1.bits == set2.bits
    }

    pub fn major(dev: ::dev_t) -> ::c_uint {
        let mut major = 0;
        major |= (dev & 0x00000000000fff00) >> 8;
        major |= (dev & 0xfffff00000000000) >> 32;
        major as ::c_uint
    }

    pub fn minor(dev: ::dev_t) -> ::c_uint {
        let mut minor = 0;
        minor |= (dev & 0x00000000000000ff) >> 0;
        minor |= (dev & 0x00000ffffff00000) >> 12;
        minor as ::c_uint
    }

    pub fn makedev(major: ::c_uint, minor: ::c_uint) -> ::dev_t {
        let major = major as ::dev_t;
        let minor = minor as ::dev_t;
        let mut dev = 0;
        dev |= (major & 0x00000fff) << 8;
        dev |= (major & 0xfffff000) << 32;
        dev |= (minor & 0x000000ff) << 0;
        dev |= (minor & 0xffffff00) << 12;
        dev
    }
}

extern {
    pub fn aio_read(aiocbp: *mut aiocb) -> ::c_int;
    pub fn aio_write(aiocbp: *mut aiocb) -> ::c_int;
    pub fn aio_fsync(op: ::c_int, aiocbp: *mut aiocb) -> ::c_int;
    pub fn aio_error(aiocbp: *const aiocb) -> ::c_int;
    pub fn aio_return(aiocbp: *mut aiocb) -> ::ssize_t;
    pub fn aio_suspend(aiocb_list: *const *const aiocb, nitems: ::c_int,
                       timeout: *const ::timespec) -> ::c_int;
    pub fn aio_cancel(fd: ::c_int, aiocbp: *mut aiocb) -> ::c_int;
    pub fn lio_listio(mode: ::c_int, aiocb_list: *const *mut aiocb,
                      nitems: ::c_int, sevp: *mut ::sigevent) -> ::c_int;

    pub fn lutimes(file: *const ::c_char, times: *const ::timeval) -> ::c_int;

    pub fn setpwent();
    pub fn endpwent();
    pub fn getpwent() -> *mut passwd;
    pub fn setgrent();
    pub fn endgrent();
    pub fn getgrent() -> *mut ::group;
    pub fn setspent();
    pub fn endspent();
    pub fn getspent() -> *mut spwd;

    pub fn getspnam(__name: *const ::c_char) -> *mut spwd;

    pub fn shm_open(name: *const c_char, oflag: ::c_int,
                    mode: mode_t) -> ::c_int;

    // System V IPC
    pub fn shmget(key: ::key_t, size: ::size_t, shmflg: ::c_int) -> ::c_int;
    pub fn shmat(shmid: ::c_int,
                 shmaddr: *const ::c_void,
                 shmflg: ::c_int) -> *mut ::c_void;
    pub fn shmdt(shmaddr: *const ::c_void) -> ::c_int;
    pub fn shmctl(shmid: ::c_int,
                  cmd: ::c_int,
                  buf: *mut ::shmid_ds) -> ::c_int;
    pub fn ftok(pathname: *const ::c_char, proj_id: ::c_int) -> ::key_t;
    pub fn semget(key: ::key_t, nsems: ::c_int, semflag: ::c_int) -> ::c_int;
    pub fn semop(semid: ::c_int,
                 sops: *mut ::sembuf, nsops: ::size_t) -> ::c_int;
    pub fn semctl(semid: ::c_int,
                  semnum: ::c_int, cmd: ::c_int, ...) -> ::c_int;
    pub fn msgctl(msqid: ::c_int, cmd: ::c_int, buf: *mut msqid_ds) -> ::c_int;
    pub fn msgget(key: ::key_t, msgflg: ::c_int) -> ::c_int;
    pub fn msgrcv(msqid: ::c_int, msgp: *mut ::c_void, msgsz: ::size_t,
                  msgtyp: ::c_long, msgflg: ::c_int) -> ::ssize_t;
    pub fn msgsnd(msqid: ::c_int, msgp: *const ::c_void, msgsz: ::size_t,
                  msgflg: ::c_int) -> ::c_int;

    pub fn mprotect(addr: *mut ::c_void, len: ::size_t, prot: ::c_int)
                    -> ::c_int;
    pub fn __errno_location() -> *mut ::c_int;

    pub fn fopen64(filename: *const c_char,
                   mode: *const c_char) -> *mut ::FILE;
    pub fn freopen64(filename: *const c_char, mode: *const c_char,
                     file: *mut ::FILE) -> *mut ::FILE;
    pub fn tmpfile64() -> *mut ::FILE;
    pub fn fgetpos64(stream: *mut ::FILE, ptr: *mut fpos64_t) -> ::c_int;
    pub fn fsetpos64(stream: *mut ::FILE, ptr: *const fpos64_t) -> ::c_int;
    pub fn fseeko64(stream: *mut ::FILE,
                    offset: ::off64_t,
                    whence: ::c_int) -> ::c_int;
    pub fn ftello64(stream: *mut ::FILE) -> ::off64_t;
    pub fn fallocate(fd: ::c_int, mode: ::c_int,
                     offset: ::off_t, len: ::off_t) -> ::c_int;
    pub fn posix_fallocate(fd: ::c_int, offset: ::off_t,
                           len: ::off_t) -> ::c_int;
    pub fn readahead(fd: ::c_int, offset: ::off64_t,
                     count: ::size_t) -> ::ssize_t;
    pub fn getxattr(path: *const c_char, name: *const c_char,
                    value: *mut ::c_void, size: ::size_t) -> ::ssize_t;
    pub fn lgetxattr(path: *const c_char, name: *const c_char,
                     value: *mut ::c_void, size: ::size_t) -> ::ssize_t;
    pub fn fgetxattr(filedes: ::c_int, name: *const c_char,
                     value: *mut ::c_void, size: ::size_t) -> ::ssize_t;
    pub fn setxattr(path: *const c_char, name: *const c_char,
                    value: *const ::c_void, size: ::size_t,
                    flags: ::c_int) -> ::c_int;
    pub fn lsetxattr(path: *const c_char, name: *const c_char,
                     value: *const ::c_void, size: ::size_t,
                     flags: ::c_int) -> ::c_int;
    pub fn fsetxattr(filedes: ::c_int, name: *const c_char,
                     value: *const ::c_void, size: ::size_t,
                     flags: ::c_int) -> ::c_int;
    pub fn listxattr(path: *const c_char, list: *mut c_char,
                     size: ::size_t) -> ::ssize_t;
    pub fn llistxattr(path: *const c_char, list: *mut c_char,
                      size: ::size_t) -> ::ssize_t;
    pub fn flistxattr(filedes: ::c_int, list: *mut c_char,
                      size: ::size_t) -> ::ssize_t;
    pub fn removexattr(path: *const c_char, name: *const c_char) -> ::c_int;
    pub fn lremovexattr(path: *const c_char, name: *const c_char) -> ::c_int;
    pub fn fremovexattr(filedes: ::c_int, name: *const c_char) -> ::c_int;
    pub fn signalfd(fd: ::c_int,
                    mask: *const ::sigset_t,
                    flags: ::c_int) -> ::c_int;
    pub fn timerfd_create(clockid: ::c_int, flags: ::c_int) -> ::c_int;
    pub fn timerfd_gettime(fd: ::c_int,
                           curr_value: *mut itimerspec) -> ::c_int;
    pub fn timerfd_settime(fd: ::c_int,
                           flags: ::c_int,
                           new_value: *const itimerspec,
                           old_value: *mut itimerspec) -> ::c_int;
    pub fn pwritev(fd: ::c_int,
                   iov: *const ::iovec,
                   iovcnt: ::c_int,
                   offset: ::off_t) -> ::ssize_t;
    pub fn preadv(fd: ::c_int,
                  iov: *const ::iovec,
                  iovcnt: ::c_int,
                  offset: ::off_t) -> ::ssize_t;
    pub fn quotactl(cmd: ::c_int,
                    special: *const ::c_char,
                    id: ::c_int,
                    data: *mut ::c_char) -> ::c_int;
    pub fn mq_open(name: *const ::c_char, oflag: ::c_int, ...) -> ::mqd_t;
    pub fn mq_close(mqd: ::mqd_t) -> ::c_int;
    pub fn mq_unlink(name: *const ::c_char) -> ::c_int;
    pub fn mq_receive(mqd: ::mqd_t,
                      msg_ptr: *mut ::c_char,
                      msg_len: ::size_t,
                      msq_prio: *mut ::c_uint) -> ::ssize_t;
    pub fn mq_send(mqd: ::mqd_t,
                   msg_ptr: *const ::c_char,
                   msg_len: ::size_t,
                   msq_prio: ::c_uint) -> ::c_int;
    pub fn mq_getattr(mqd: ::mqd_t, attr: *mut ::mq_attr) -> ::c_int;
    pub fn mq_setattr(mqd: ::mqd_t,
                      newattr: *const ::mq_attr,
                      oldattr: *mut ::mq_attr) -> ::c_int;
    pub fn epoll_pwait(epfd: ::c_int,
                       events: *mut ::epoll_event,
                       maxevents: ::c_int,
                       timeout: ::c_int,
                       sigmask: *const ::sigset_t) -> ::c_int;
    pub fn dup3(oldfd: ::c_int, newfd: ::c_int, flags: ::c_int) -> ::c_int;
    pub fn mkostemp(template: *mut ::c_char, flags: ::c_int) -> ::c_int;
    pub fn mkostemps(template: *mut ::c_char,
                     suffixlen: ::c_int,
                     flags: ::c_int) -> ::c_int;
    pub fn sigtimedwait(set: *const sigset_t,
                        info: *mut siginfo_t,
                        timeout: *const ::timespec) -> ::c_int;
    pub fn sigwaitinfo(set: *const sigset_t,
                       info: *mut siginfo_t) -> ::c_int;
    pub fn nl_langinfo_l(item: ::nl_item, locale: ::locale_t) -> *mut ::c_char;
    pub fn getnameinfo(sa: *const ::sockaddr,
                       salen: ::socklen_t,
                       host: *mut ::c_char,
                       hostlen: ::socklen_t,
                       serv: *mut ::c_char,
                       sevlen: ::socklen_t,
                       flags: ::c_int) -> ::c_int;
    pub fn pthread_setschedprio(native: ::pthread_t,
                                priority: ::c_int) -> ::c_int;
    pub fn prlimit(pid: ::pid_t, resource: ::c_int, new_limit: *const ::rlimit,
                   old_limit: *mut ::rlimit) -> ::c_int;
    pub fn prlimit64(pid: ::pid_t,
                     resource: ::c_int,
                     new_limit: *const ::rlimit64,
                     old_limit: *mut ::rlimit64) -> ::c_int;
    pub fn getloadavg(loadavg: *mut ::c_double, nelem: ::c_int) -> ::c_int;
    pub fn process_vm_readv(pid: ::pid_t,
                            local_iov: *const ::iovec,
                            liovcnt: ::c_ulong,
                            remote_iov: *const ::iovec,
                            riovcnt: ::c_ulong,
                            flags: ::c_ulong) -> isize;
    pub fn process_vm_writev(pid: ::pid_t,
                             local_iov: *const ::iovec,
                             liovcnt: ::c_ulong,
                             remote_iov: *const ::iovec,
                             riovcnt: ::c_ulong,
                             flags: ::c_ulong) -> isize;
    pub fn reboot(how_to: ::c_int) -> ::c_int;
    pub fn setfsgid(gid: ::gid_t) -> ::c_int;
    pub fn setfsuid(uid: ::uid_t) -> ::c_int;

    // Not available now on Android
    pub fn mkfifoat(dirfd: ::c_int, pathname: *const ::c_char,
                    mode: ::mode_t) -> ::c_int;
    pub fn if_nameindex() -> *mut if_nameindex;
    pub fn if_freenameindex(ptr: *mut if_nameindex);
    pub fn sync_file_range(fd: ::c_int, offset: ::off64_t,
                           nbytes: ::off64_t, flags: ::c_uint) -> ::c_int;
    pub fn getifaddrs(ifap: *mut *mut ::ifaddrs) -> ::c_int;
    pub fn freeifaddrs(ifa: *mut ::ifaddrs);
    pub fn mremap(addr: *mut ::c_void,
                  len: ::size_t,
                  new_len: ::size_t,
                  flags: ::c_int,
                  ...) -> *mut ::c_void;

    pub fn glob(pattern: *const c_char,
                flags: ::c_int,
                errfunc: Option<extern fn(epath: *const c_char,
                                          errno: ::c_int) -> ::c_int>,
                pglob: *mut ::glob_t) -> ::c_int;
    pub fn globfree(pglob: *mut ::glob_t);

    pub fn posix_madvise(addr: *mut ::c_void, len: ::size_t, advice: ::c_int)
                         -> ::c_int;

    pub fn shm_unlink(name: *const ::c_char) -> ::c_int;

    pub fn seekdir(dirp: *mut ::DIR, loc: ::c_long);

    pub fn telldir(dirp: *mut ::DIR) -> ::c_long;
    pub fn madvise(addr: *mut ::c_void, len: ::size_t, advice: ::c_int)
                  -> ::c_int;

    pub fn msync(addr: *mut ::c_void, len: ::size_t, flags: ::c_int) -> ::c_int;
    pub fn remap_file_pages(addr: *mut ::c_void, size: ::size_t, prot: ::c_int,
                            pgoff: ::size_t, flags: ::c_int) -> ::c_int;
    pub fn recvfrom(socket: ::c_int, buf: *mut ::c_void, len: ::size_t,
                    flags: ::c_int, addr: *mut ::sockaddr,
                    addrlen: *mut ::socklen_t) -> ::ssize_t;
    pub fn mkstemps(template: *mut ::c_char, suffixlen: ::c_int) -> ::c_int;
    pub fn futimes(fd: ::c_int, times: *const ::timeval) -> ::c_int;
    pub fn nl_langinfo(item: ::nl_item) -> *mut ::c_char;

    pub fn bind(socket: ::c_int, address: *const ::sockaddr,
                address_len: ::socklen_t) -> ::c_int;

    pub fn writev(fd: ::c_int,
                  iov: *const ::iovec,
                  iovcnt: ::c_int) -> ::ssize_t;
    pub fn readv(fd: ::c_int,
                 iov: *const ::iovec,
                 iovcnt: ::c_int) -> ::ssize_t;

    pub fn sendmsg(fd: ::c_int,
                   msg: *const ::msghdr,
                   flags: ::c_int) -> ::ssize_t;
    pub fn recvmsg(fd: ::c_int, msg: *mut ::msghdr, flags: ::c_int)
                   -> ::ssize_t;
    pub fn getdomainname(name: *mut ::c_char, len: ::size_t) -> ::c_int;
    pub fn setdomainname(name: *const ::c_char, len: ::size_t) -> ::c_int;
    pub fn vhangup() -> ::c_int;
    pub fn sendmmsg(sockfd: ::c_int, msgvec: *mut mmsghdr, vlen: ::c_uint,
                    flags: ::c_int) -> ::c_int;
    pub fn recvmmsg(sockfd: ::c_int, msgvec: *mut mmsghdr, vlen: ::c_uint,
                    flags: ::c_int, timeout: *mut ::timespec) -> ::c_int;
    pub fn sync();
    pub fn syscall(num: ::c_long, ...) -> ::c_long;
    pub fn sched_getaffinity(pid: ::pid_t,
                             cpusetsize: ::size_t,
                             cpuset: *mut cpu_set_t) -> ::c_int;
    pub fn sched_setaffinity(pid: ::pid_t,
                             cpusetsize: ::size_t,
                             cpuset: *const cpu_set_t) -> ::c_int;
    pub fn epoll_create(size: ::c_int) -> ::c_int;
    pub fn epoll_create1(flags: ::c_int) -> ::c_int;
    pub fn epoll_wait(epfd: ::c_int,
                      events: *mut ::epoll_event,
                      maxevents: ::c_int,
                      timeout: ::c_int) -> ::c_int;
    pub fn epoll_ctl(epfd: ::c_int,
                     op: ::c_int,
                     fd: ::c_int,
                     event: *mut ::epoll_event) -> ::c_int;
    pub fn pthread_getschedparam(native: ::pthread_t,
                                 policy: *mut ::c_int,
                                 param: *mut ::sched_param) -> ::c_int;
    pub fn unshare(flags: ::c_int) -> ::c_int;
    pub fn umount(target: *const ::c_char) -> ::c_int;
    pub fn sched_get_priority_max(policy: ::c_int) -> ::c_int;
    pub fn tee(fd_in: ::c_int,
               fd_out: ::c_int,
               len: ::size_t,
               flags: ::c_uint) -> ::ssize_t;
    pub fn settimeofday(tv: *const ::timeval, tz: *const ::timezone) -> ::c_int;
    pub fn splice(fd_in: ::c_int,
                  off_in: *mut ::loff_t,
                  fd_out: ::c_int,
                  off_out: *mut ::loff_t,
                  len: ::size_t,
                  flags: ::c_uint) -> ::ssize_t;
    pub fn eventfd(init: ::c_uint, flags: ::c_int) -> ::c_int;
    pub fn sched_rr_get_interval(pid: ::pid_t, tp: *mut ::timespec) -> ::c_int;
    pub fn sem_timedwait(sem: *mut sem_t,
                         abstime: *const ::timespec) -> ::c_int;
    pub fn sched_setparam(pid: ::pid_t, param: *const ::sched_param) -> ::c_int;
    pub fn setns(fd: ::c_int, nstype: ::c_int) -> ::c_int;
    pub fn swapoff(puath: *const ::c_char) -> ::c_int;
    pub fn vmsplice(fd: ::c_int,
                    iov: *const ::iovec,
                    nr_segs: ::size_t,
                    flags: ::c_uint) -> ::ssize_t;
    pub fn mount(src: *const ::c_char,
                 target: *const ::c_char,
                 fstype: *const ::c_char,
                 flags: ::c_ulong,
                 data: *const ::c_void) -> ::c_int;
    pub fn personality(persona: ::c_ulong) -> ::c_int;
    pub fn prctl(option: ::c_int, ...) -> ::c_int;
    pub fn sched_getparam(pid: ::pid_t, param: *mut ::sched_param) -> ::c_int;
    pub fn ppoll(fds: *mut ::pollfd,
                 nfds: nfds_t,
                 timeout: *const ::timespec,
                 sigmask: *const sigset_t) -> ::c_int;
    pub fn pthread_mutex_timedlock(lock: *mut pthread_mutex_t,
                                   abstime: *const ::timespec) -> ::c_int;
    pub fn clone(cb: extern fn(*mut ::c_void) -> ::c_int,
                 child_stack: *mut ::c_void,
                 flags: ::c_int,
                 arg: *mut ::c_void, ...) -> ::c_int;
    pub fn sched_getscheduler(pid: ::pid_t) -> ::c_int;
    pub fn clock_nanosleep(clk_id: ::clockid_t,
                           flags: ::c_int,
                           rqtp: *const ::timespec,
                           rmtp:  *mut ::timespec) -> ::c_int;
    pub fn pthread_attr_getguardsize(attr: *const ::pthread_attr_t,
                                     guardsize: *mut ::size_t) -> ::c_int;
    pub fn sethostname(name: *const ::c_char, len: ::size_t) -> ::c_int;
    pub fn sched_get_priority_min(policy: ::c_int) -> ::c_int;
    pub fn pthread_condattr_getpshared(attr: *const pthread_condattr_t,
                                       pshared: *mut ::c_int) -> ::c_int;
    pub fn sysinfo(info: *mut ::sysinfo) -> ::c_int;
    pub fn umount2(target: *const ::c_char, flags: ::c_int) -> ::c_int;
    pub fn pthread_setschedparam(native: ::pthread_t,
                                 policy: ::c_int,
                                 param: *const ::sched_param) -> ::c_int;
    pub fn swapon(path: *const ::c_char, swapflags: ::c_int) -> ::c_int;
    pub fn sched_setscheduler(pid: ::pid_t,
                              policy: ::c_int,
                              param: *const ::sched_param) -> ::c_int;
    pub fn sendfile(out_fd: ::c_int,
                    in_fd: ::c_int,
                    offset: *mut off_t,
                    count: ::size_t) -> ::ssize_t;
    pub fn sigsuspend(mask: *const ::sigset_t) -> ::c_int;
    #[cfg_attr(target_os = "solaris", link_name = "__posix_getgrgid_r")]
    pub fn getgrgid_r(uid: ::uid_t,
                      grp: *mut ::group,
                      buf: *mut ::c_char,
                      buflen: ::size_t,
                      result: *mut *mut ::group) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "sigaltstack$UNIX2003")]
    #[cfg_attr(target_os = "netbsd", link_name = "__sigaltstack14")]
    pub fn sigaltstack(ss: *const stack_t,
                       oss: *mut stack_t) -> ::c_int;
    pub fn sem_close(sem: *mut sem_t) -> ::c_int;
    pub fn getdtablesize() -> ::c_int;
    #[cfg_attr(target_os = "solaris", link_name = "__posix_getgrnam_r")]
    pub fn getgrnam_r(name: *const ::c_char,
                      grp: *mut ::group,
                      buf: *mut ::c_char,
                      buflen: ::size_t,
                      result: *mut *mut ::group) -> ::c_int;
    pub fn initgroups(user: *const ::c_char, group: ::gid_t) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "pthread_sigmask$UNIX2003")]
    pub fn pthread_sigmask(how: ::c_int, set: *const sigset_t,
                           oldset: *mut sigset_t) -> ::c_int;
    pub fn sem_open(name: *const ::c_char, oflag: ::c_int, ...) -> *mut sem_t;
    pub fn getgrnam(name: *const ::c_char) -> *mut ::group;
    pub fn pthread_cancel(thread: ::pthread_t) -> ::c_int;
    pub fn pthread_kill(thread: ::pthread_t, sig: ::c_int) -> ::c_int;
    pub fn sem_unlink(name: *const ::c_char) -> ::c_int;
    pub fn daemon(nochdir: ::c_int, noclose: ::c_int) -> ::c_int;
    #[cfg_attr(target_os = "netbsd", link_name = "__getpwnam_r50")]
    #[cfg_attr(target_os = "solaris", link_name = "__posix_getpwnam_r")]
    pub fn getpwnam_r(name: *const ::c_char,
                      pwd: *mut passwd,
                      buf: *mut ::c_char,
                      buflen: ::size_t,
                      result: *mut *mut passwd) -> ::c_int;
    #[cfg_attr(target_os = "netbsd", link_name = "__getpwuid_r50")]
    #[cfg_attr(target_os = "solaris", link_name = "__posix_getpwuid_r")]
    pub fn getpwuid_r(uid: ::uid_t,
                      pwd: *mut passwd,
                      buf: *mut ::c_char,
                      buflen: ::size_t,
                      result: *mut *mut passwd) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch ="x86"),
               link_name = "sigwait$UNIX2003")]
    #[cfg_attr(target_os = "solaris", link_name = "__posix_sigwait")]
    pub fn sigwait(set: *const sigset_t,
                   sig: *mut ::c_int) -> ::c_int;
    pub fn pthread_atfork(prepare: Option<unsafe extern fn()>,
                          parent: Option<unsafe extern fn()>,
                          child: Option<unsafe extern fn()>) -> ::c_int;
    pub fn getgrgid(gid: ::gid_t) -> *mut ::group;
    pub fn getgrouplist(user: *const ::c_char,
                        group: ::gid_t,
                        groups: *mut ::gid_t,
                        ngroups: *mut ::c_int) -> ::c_int;
    pub fn pthread_mutexattr_getpshared(attr: *const pthread_mutexattr_t,
                                        pshared: *mut ::c_int) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "popen$UNIX2003")]
    pub fn popen(command: *const c_char,
                 mode: *const c_char) -> *mut ::FILE;
    pub fn faccessat(dirfd: ::c_int, pathname: *const ::c_char,
                     mode: ::c_int, flags: ::c_int) -> ::c_int;
    pub fn pthread_create(native: *mut ::pthread_t,
                          attr: *const ::pthread_attr_t,
                          f: extern fn(*mut ::c_void) -> *mut ::c_void,
                          value: *mut ::c_void) -> ::c_int;
    pub fn dl_iterate_phdr(
        callback: Option<unsafe extern fn(
            info: *mut ::dl_phdr_info,
            size: ::size_t,
            data: *mut ::c_void
        ) -> ::c_int>,
        data: *mut ::c_void
    ) -> ::c_int;

    pub fn setmntent(filename: *const ::c_char,
                     ty: *const ::c_char) -> *mut ::FILE;
    pub fn getmntent(stream: *mut ::FILE) -> *mut ::mntent;
    pub fn addmntent(stream: *mut ::FILE, mnt: *const ::mntent) -> ::c_int;
    pub fn endmntent(streamp: *mut ::FILE) -> ::c_int;
    pub fn hasmntopt(mnt: *const ::mntent,
                     opt: *const ::c_char) -> *mut ::c_char;

    pub fn posix_spawn(pid: *mut ::pid_t,
                       path: *const ::c_char,
                       file_actions: *const ::posix_spawn_file_actions_t,
                       attrp: *const ::posix_spawnattr_t,
                       argv: *const *mut ::c_char,
                       envp: *const *mut ::c_char) -> ::c_int;
    pub fn posix_spawnp(pid: *mut ::pid_t,
                       file: *const ::c_char,
                        file_actions: *const ::posix_spawn_file_actions_t,
                        attrp: *const ::posix_spawnattr_t,
                        argv: *const *mut ::c_char,
                        envp: *const *mut ::c_char) -> ::c_int;
    pub fn posix_spawnattr_init(attr: *mut posix_spawnattr_t) -> ::c_int;
    pub fn posix_spawnattr_destroy(attr: *mut posix_spawnattr_t) -> ::c_int;
    pub fn posix_spawnattr_getsigdefault(attr: *const posix_spawnattr_t,
                                         default: *mut ::sigset_t) -> ::c_int;
    pub fn posix_spawnattr_setsigdefault(attr: *mut posix_spawnattr_t,
                                         default: *const ::sigset_t) -> ::c_int;
    pub fn posix_spawnattr_getsigmask(attr: *const posix_spawnattr_t,
                                      default: *mut ::sigset_t) -> ::c_int;
    pub fn posix_spawnattr_setsigmask(attr: *mut posix_spawnattr_t,
                                      default: *const ::sigset_t) -> ::c_int;
    pub fn posix_spawnattr_getflags(attr: *const posix_spawnattr_t,
                                    flags: *mut ::c_short) -> ::c_int;
    pub fn posix_spawnattr_setflags(attr: *mut posix_spawnattr_t,
                                    flags: ::c_short) -> ::c_int;
    pub fn posix_spawnattr_getpgroup(attr: *const posix_spawnattr_t,
                                     flags: *mut ::pid_t) -> ::c_int;
    pub fn posix_spawnattr_setpgroup(attr: *mut posix_spawnattr_t,
                                     flags: ::pid_t) -> ::c_int;
    pub fn posix_spawnattr_getschedpolicy(attr: *const posix_spawnattr_t,
                                          flags: *mut ::c_int) -> ::c_int;
    pub fn posix_spawnattr_setschedpolicy(attr: *mut posix_spawnattr_t,
                                          flags: ::c_int) -> ::c_int;
    pub fn posix_spawnattr_getschedparam(
        attr: *const posix_spawnattr_t,
        param: *mut ::sched_param,
    ) -> ::c_int;
    pub fn posix_spawnattr_setschedparam(
        attr: *mut posix_spawnattr_t,
        param: *const ::sched_param,
    ) -> ::c_int;

    pub fn posix_spawn_file_actions_init(
        actions: *mut posix_spawn_file_actions_t,
    ) -> ::c_int;
    pub fn posix_spawn_file_actions_destroy(
        actions: *mut posix_spawn_file_actions_t,
    ) -> ::c_int;
    pub fn posix_spawn_file_actions_addopen(
        actions: *mut posix_spawn_file_actions_t,
        fd: ::c_int,
        path: *const ::c_char,
        oflag: ::c_int,
        mode: ::mode_t,
    ) -> ::c_int;
    pub fn posix_spawn_file_actions_addclose(
        actions: *mut posix_spawn_file_actions_t,
        fd: ::c_int,
    ) -> ::c_int;
    pub fn posix_spawn_file_actions_adddup2(
        actions: *mut posix_spawn_file_actions_t,
        fd: ::c_int,
        newfd: ::c_int,
    ) -> ::c_int;
}

cfg_if! {
    if #[cfg(any(target_env = "musl", target_os = "fuchsia"))] {
        mod musl;
        pub use self::musl::*;
    } else if #[cfg(any(target_arch = "mips",
                        target_arch = "mips64"))] {
        mod mips;
        pub use self::mips::*;
    } else if #[cfg(any(target_arch = "s390x"))] {
        mod s390x;
        pub use self::s390x::*;
    } else {
        mod other;
        pub use self::other::*;
    }
}
