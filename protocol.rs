// This file is generated. Do not edit

use protobuf::*;
use protobuf::rt;
use protobuf::descriptor;

static file_descriptor_proto_data: &'static [u8] = &[0x0a, 0x0e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x02, 0x71, 0x32, 0x22, 0xa4, 0x03, 0x0a, 0x0d, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x12, 0x2b, 0x0a, 0x0b, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x16, 0x2e, 0x71, 0x32, 0x2e, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x12, 0x12, 0x0a, 0x0a, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x12, 0x11, 0x0a, 0x09, 0x62, 0x75, 0x66, 0x66, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x12, 0x2a, 0x0a, 0x0e, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x5f, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x71, 0x32, 0x2e, 0x41, 0x74, 0x74, 0x61, 0x63, 0x68, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x54, 0x12, 0x1d, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x71, 0x32, 0x2e, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x54, 0x12, 0x1f, 0x0a, 0x08, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x71, 0x32, 0x2e, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x54, 0x12, 0x26, 0x0a, 0x0c, 0x6a, 0x6f, 0x69, 0x6e, 0x5f, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x71, 0x32, 0x2e, 0x4a, 0x6f, 0x69, 0x6e, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x54, 0x12, 0x26, 0x0a, 0x0c, 0x73, 0x65, 0x6e, 0x64, 0x5f, 0x70, 0x72, 0x69, 0x76, 0x6d, 0x73, 0x67, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x71, 0x32, 0x2e, 0x53, 0x65, 0x6e, 0x64, 0x50, 0x72, 0x69, 0x76, 0x6d, 0x73, 0x67, 0x54, 0x22, 0x82, 0x01, 0x0a, 0x04, 0x54, 0x79, 0x70, 0x65, 0x12, 0x11, 0x0a, 0x0d, 0x41, 0x74, 0x74, 0x61, 0x63, 0x68, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x10, 0x01, 0x12, 0x12, 0x0a, 0x0e, 0x47, 0x65, 0x74, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x4c, 0x69, 0x73, 0x74, 0x10, 0x64, 0x12, 0x0c, 0x0a, 0x07, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x10, 0xc8, 0x01, 0x12, 0x0d, 0x0a, 0x08, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x10, 0xc9, 0x01, 0x12, 0x10, 0x0a, 0x0b, 0x4a, 0x6f, 0x69, 0x6e, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x10, 0xca, 0x01, 0x12, 0x10, 0x0a, 0x0b, 0x53, 0x65, 0x6e, 0x64, 0x50, 0x72, 0x69, 0x76, 0x6d, 0x73, 0x67, 0x10, 0xcb, 0x01, 0x12, 0x12, 0x0a, 0x0d, 0x47, 0x65, 0x74, 0x42, 0x75, 0x66, 0x66, 0x65, 0x72, 0x4c, 0x69, 0x73, 0x74, 0x10, 0xcc, 0x01, 0x22, 0x24, 0x0a, 0x0e, 0x41, 0x74, 0x74, 0x61, 0x63, 0x68, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x54, 0x12, 0x12, 0x0a, 0x0a, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x22, 0x1b, 0x0a, 0x08, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x54, 0x12, 0x0f, 0x0a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x22, 0x1d, 0x0a, 0x09, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x54, 0x12, 0x10, 0x0a, 0x08, 0x6e, 0x69, 0x63, 0x6b, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x22, 0x1f, 0x0a, 0x0c, 0x4a, 0x6f, 0x69, 0x6e, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x54, 0x12, 0x0f, 0x0a, 0x07, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x22, 0x2b, 0x0a, 0x0c, 0x53, 0x65, 0x6e, 0x64, 0x50, 0x72, 0x69, 0x76, 0x6d, 0x73, 0x67, 0x54, 0x12, 0x0e, 0x0a, 0x06, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0b, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x18, 0x03, 0x20, 0x02, 0x28, 0x09, 0x22, 0xbe, 0x03, 0x0a, 0x0d, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x2b, 0x0a, 0x0b, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x16, 0x2e, 0x71, 0x32, 0x2e, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x12, 0x12, 0x0a, 0x0a, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x12, 0x11, 0x0a, 0x09, 0x62, 0x75, 0x66, 0x66, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x12, 0x12, 0x0a, 0x0a, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x12, 0x14, 0x0a, 0x0c, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x04, 0x12, 0x26, 0x0a, 0x0c, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x6c, 0x69, 0x73, 0x74, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x71, 0x32, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x12, 0x27, 0x0a, 0x0c, 0x64, 0x69, 0x73, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x65, 0x64, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x71, 0x32, 0x2e, 0x44, 0x69, 0x73, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x65, 0x64, 0x54, 0x12, 0x24, 0x0a, 0x0b, 0x62, 0x75, 0x66, 0x66, 0x65, 0x72, 0x5f, 0x6c, 0x69, 0x73, 0x74, 0x18, 0x09, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x71, 0x32, 0x2e, 0x42, 0x75, 0x66, 0x66, 0x65, 0x72, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x12, 0x22, 0x0a, 0x0a, 0x6e, 0x65, 0x77, 0x5f, 0x62, 0x75, 0x66, 0x66, 0x65, 0x72, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x71, 0x32, 0x2e, 0x4e, 0x65, 0x77, 0x42, 0x75, 0x66, 0x66, 0x65, 0x72, 0x54, 0x12, 0x25, 0x0a, 0x0b, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x71, 0x32, 0x2e, 0x49, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x54, 0x22, 0x6d, 0x0a, 0x04, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0f, 0x0a, 0x0b, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x4c, 0x69, 0x73, 0x74, 0x10, 0x64, 0x12, 0x0e, 0x0a, 0x09, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x65, 0x64, 0x10, 0xc9, 0x01, 0x12, 0x11, 0x0a, 0x0c, 0x44, 0x69, 0x73, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x65, 0x64, 0x10, 0xca, 0x01, 0x12, 0x0f, 0x0a, 0x0a, 0x42, 0x75, 0x66, 0x66, 0x65, 0x72, 0x4c, 0x69, 0x73, 0x74, 0x10, 0xcb, 0x01, 0x12, 0x0e, 0x0a, 0x09, 0x4e, 0x65, 0x77, 0x42, 0x75, 0x66, 0x66, 0x65, 0x72, 0x10, 0xcc, 0x01, 0x12, 0x10, 0x0a, 0x0b, 0x49, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x10, 0xcd, 0x01, 0x22, 0x1a, 0x0a, 0x0c, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x22, 0x1f, 0x0a, 0x0d, 0x44, 0x69, 0x73, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x65, 0x64, 0x54, 0x12, 0x0e, 0x0a, 0x06, 0x72, 0x65, 0x61, 0x73, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x22, 0x70, 0x0a, 0x0a, 0x42, 0x75, 0x66, 0x66, 0x65, 0x72, 0x52, 0x6f, 0x6c, 0x65, 0x12, 0x28, 0x0a, 0x0b, 0x62, 0x75, 0x66, 0x66, 0x65, 0x72, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x13, 0x2e, 0x71, 0x32, 0x2e, 0x42, 0x75, 0x66, 0x66, 0x65, 0x72, 0x52, 0x6f, 0x6c, 0x65, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x22, 0x2a, 0x0a, 0x04, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0a, 0x0a, 0x06, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x10, 0x01, 0x12, 0x0b, 0x0a, 0x07, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x10, 0x02, 0x12, 0x09, 0x0a, 0x05, 0x51, 0x75, 0x65, 0x72, 0x79, 0x10, 0x03, 0x22, 0x37, 0x0a, 0x0b, 0x42, 0x75, 0x66, 0x66, 0x65, 0x72, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x1c, 0x0a, 0x04, 0x72, 0x6f, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x71, 0x32, 0x2e, 0x42, 0x75, 0x66, 0x66, 0x65, 0x72, 0x52, 0x6f, 0x6c, 0x65, 0x22, 0x36, 0x0a, 0x0a, 0x4e, 0x65, 0x77, 0x42, 0x75, 0x66, 0x66, 0x65, 0x72, 0x54, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x1c, 0x0a, 0x04, 0x72, 0x6f, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x71, 0x32, 0x2e, 0x42, 0x75, 0x66, 0x66, 0x65, 0x72, 0x52, 0x6f, 0x6c, 0x65, 0x22, 0x1b, 0x0a, 0x0c, 0x49, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x54, 0x12, 0x0b, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x4a, 0xf2, 0x1b, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x6c, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x0a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x02, 0x00, 0x1a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x02, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x04, 0x00, 0x12, 0x04, 0x03, 0x02, 0x0e, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04, 0x00, 0x01, 0x12, 0x03, 0x03, 0x07, 0x0b, 0x0a, 0x1f, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x04, 0x16, 0x1a, 0x10, 0x20, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x04, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x05, 0x14, 0x15, 0x0a, 0x18, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x07, 0x04, 0x19, 0x1a, 0x09, 0x20, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x07, 0x04, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x07, 0x15, 0x18, 0x0a, 0x18, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x09, 0x04, 0x12, 0x1a, 0x09, 0x20, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x09, 0x04, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x09, 0x0e, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0a, 0x04, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0a, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x0a, 0x0f, 0x12, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0b, 0x04, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0b, 0x04, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x0b, 0x12, 0x15, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x0c, 0x04, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0c, 0x04, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x0c, 0x12, 0x15, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x0d, 0x04, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x0d, 0x04, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x0d, 0x14, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x10, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x10, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x10, 0x10, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x10, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x12, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x12, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x12, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x12, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x13, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x13, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x13, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x13, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x15, 0x02, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x15, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x15, 0x0b, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x15, 0x1a, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x15, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x16, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x16, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x16, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x16, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x16, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x17, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x17, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x06, 0x12, 0x03, 0x17, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x17, 0x15, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x17, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x18, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x18, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x06, 0x12, 0x03, 0x18, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x18, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x18, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x19, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x19, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x06, 0x12, 0x03, 0x19, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x19, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x19, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x1c, 0x00, 0x1e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x1c, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1d, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x20, 0x00, 0x22, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x20, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x21, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x21, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x21, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x21, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x24, 0x00, 0x26, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x24, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x25, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x25, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x25, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x25, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x25, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x28, 0x00, 0x2a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x28, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x29, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x29, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x29, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x2c, 0x00, 0x2f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x2c, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x2d, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2d, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2d, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x2e, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2e, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2e, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x31, 0x00, 0x4c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x31, 0x08, 0x15, 0x0a, 0x1e, 0x0a, 0x04, 0x04, 0x06, 0x04, 0x00, 0x12, 0x04, 0x32, 0x02, 0x3d, 0x03, 0x22, 0x10, 0x20, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x04, 0x00, 0x01, 0x12, 0x03, 0x32, 0x07, 0x0b, 0x0a, 0x18, 0x0a, 0x06, 0x04, 0x06, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x35, 0x04, 0x16, 0x1a, 0x09, 0x20, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x35, 0x04, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x35, 0x12, 0x15, 0x0a, 0x18, 0x0a, 0x06, 0x04, 0x06, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x37, 0x04, 0x14, 0x1a, 0x09, 0x20, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x37, 0x04, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x37, 0x10, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x06, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x38, 0x04, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x38, 0x04, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x38, 0x13, 0x16, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x06, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x39, 0x04, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x39, 0x04, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x39, 0x11, 0x14, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x06, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x3a, 0x04, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x3a, 0x04, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x3a, 0x10, 0x13, 0x0a, 0x17, 0x0a, 0x06, 0x04, 0x06, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x3c, 0x04, 0x16, 0x1a, 0x08, 0x20, 0x42, 0x75, 0x66, 0x66, 0x65, 0x72, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x3c, 0x04, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x3c, 0x12, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x3f, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3f, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3f, 0x10, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3f, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x41, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x41, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03, 0x41, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x41, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x41, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x42, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x03, 0x42, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x05, 0x12, 0x03, 0x42, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x42, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x42, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x43, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x03, 0x43, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x05, 0x12, 0x03, 0x43, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x43, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x03, 0x43, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x04, 0x12, 0x03, 0x44, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x04, 0x12, 0x03, 0x44, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x05, 0x12, 0x03, 0x44, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x01, 0x12, 0x03, 0x44, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x03, 0x12, 0x03, 0x44, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x05, 0x12, 0x03, 0x46, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x04, 0x12, 0x03, 0x46, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x06, 0x12, 0x03, 0x46, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x01, 0x12, 0x03, 0x46, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x03, 0x12, 0x03, 0x46, 0x27, 0x28, 0x0a, 0x30, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x06, 0x12, 0x03, 0x48, 0x02, 0x2a, 0x1a, 0x23, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x65, 0x64, 0x54, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x65, 0x64, 0x20, 0x3d, 0x20, 0x37, 0x3b, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x04, 0x12, 0x03, 0x48, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x06, 0x12, 0x03, 0x48, 0x0b, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x01, 0x12, 0x03, 0x48, 0x19, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x03, 0x12, 0x03, 0x48, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x07, 0x12, 0x03, 0x49, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x04, 0x12, 0x03, 0x49, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x06, 0x12, 0x03, 0x49, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x01, 0x12, 0x03, 0x49, 0x17, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x03, 0x12, 0x03, 0x49, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x08, 0x12, 0x03, 0x4a, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x04, 0x12, 0x03, 0x4a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x06, 0x12, 0x03, 0x4a, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x01, 0x12, 0x03, 0x4a, 0x16, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x03, 0x12, 0x03, 0x4a, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x09, 0x12, 0x03, 0x4b, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x09, 0x04, 0x12, 0x03, 0x4b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x09, 0x06, 0x12, 0x03, 0x4b, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x09, 0x01, 0x12, 0x03, 0x4b, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x09, 0x03, 0x12, 0x03, 0x4b, 0x26, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x4e, 0x00, 0x50, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x4e, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x4f, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x4f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4f, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4f, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x52, 0x00, 0x54, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x52, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x53, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x53, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x53, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x53, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x53, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x56, 0x00, 0x5e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x56, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x04, 0x00, 0x12, 0x04, 0x57, 0x02, 0x5b, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x04, 0x00, 0x01, 0x12, 0x03, 0x57, 0x07, 0x0b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x09, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x58, 0x04, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x58, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x58, 0x0d, 0x0e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x09, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x59, 0x04, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x59, 0x04, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x59, 0x0e, 0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x09, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x5a, 0x04, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5a, 0x04, 0x09, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x5a, 0x0c, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x5c, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x06, 0x12, 0x03, 0x5c, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5c, 0x10, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5c, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x5d, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x03, 0x5d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x05, 0x12, 0x03, 0x5d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5d, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5d, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x60, 0x00, 0x63, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x60, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x61, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x61, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x03, 0x61, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x61, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x61, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x03, 0x62, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x04, 0x12, 0x03, 0x62, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x06, 0x12, 0x03, 0x62, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x03, 0x62, 0x16, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x03, 0x62, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x65, 0x00, 0x68, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x65, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x66, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03, 0x66, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x03, 0x66, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x66, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x03, 0x66, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x03, 0x67, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x03, 0x67, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x06, 0x12, 0x03, 0x67, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x03, 0x67, 0x16, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x03, 0x67, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x6a, 0x00, 0x6c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x6a, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x03, 0x6b, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x05, 0x12, 0x03, 0x6b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6b, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6b, 0x18, 0x19];

pub fn file_descriptor_proto() -> descriptor::FileDescriptorProto {
    parse_from_bytes(file_descriptor_proto_data)
}

#[deriving(Clone,Eq)]
pub struct RemoteCommand {
    packet_type: Option<RemoteCommand_Type>,
    network_id: Option<u64>,
    buffer_id: Option<u64>,
    attach_session: Option<AttachSessionT>,
    connect: Option<ConnectT>,
    register: Option<RegisterT>,
    join_channel: Option<JoinChannelT>,
    send_privmsg: Option<SendPrivmsgT>,
}

impl<'a> RemoteCommand {
    pub fn new() -> RemoteCommand {
        RemoteCommand {
            packet_type: None,
            network_id: None,
            buffer_id: None,
            attach_session: None,
            connect: None,
            register: None,
            join_channel: None,
            send_privmsg: None,
        }
    }

    pub fn default_instance() -> &'static RemoteCommand {
        static instance: RemoteCommand = RemoteCommand {
            packet_type: None,
            network_id: None,
            buffer_id: None,
            attach_session: None,
            connect: None,
            register: None,
            join_channel: None,
            send_privmsg: None,
        };
        &'static instance
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.packet_type {
            Some(ref v) => {
                os.write_enum(1, *v as i32);
            },
            None => {},
        };
        match self.network_id {
            Some(ref v) => {
                os.write_uint64(2, *v);
            },
            None => {},
        };
        match self.buffer_id {
            Some(ref v) => {
                os.write_uint64(3, *v);
            },
            None => {},
        };
        match self.attach_session {
            Some(ref v) => {
                os.write_tag(4, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
        match self.connect {
            Some(ref v) => {
                os.write_tag(5, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
        match self.register {
            Some(ref v) => {
                os.write_tag(6, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
        match self.join_channel {
            Some(ref v) => {
                os.write_tag(7, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
        match self.send_privmsg {
            Some(ref v) => {
                os.write_tag(8, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
    }

    pub fn clear_packet_type(&mut self) {
        self.packet_type = None;
    }

    pub fn has_packet_type(&self) -> bool {
        self.packet_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packet_type(&mut self, v: RemoteCommand_Type) {
        self.packet_type = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_packet_type(&'a mut self) -> &'a mut RemoteCommand_Type {
        if self.packet_type.is_none() {
            self.packet_type = Some(RemoteCommand_Type::new(0));
        };
        self.packet_type.get_mut_ref()
    }

    pub fn get_packet_type(&self) -> RemoteCommand_Type {
        self.packet_type.unwrap_or(RemoteCommand_Type::new(0))
    }

    pub fn clear_network_id(&mut self) {
        self.network_id = None;
    }

    pub fn has_network_id(&self) -> bool {
        self.network_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_network_id(&mut self, v: u64) {
        self.network_id = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_network_id(&'a mut self) -> &'a mut u64 {
        if self.network_id.is_none() {
            self.network_id = Some(0);
        };
        self.network_id.get_mut_ref()
    }

    pub fn get_network_id(&self) -> u64 {
        self.network_id.unwrap_or(0)
    }

    pub fn clear_buffer_id(&mut self) {
        self.buffer_id = None;
    }

    pub fn has_buffer_id(&self) -> bool {
        self.buffer_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_buffer_id(&mut self, v: u64) {
        self.buffer_id = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_buffer_id(&'a mut self) -> &'a mut u64 {
        if self.buffer_id.is_none() {
            self.buffer_id = Some(0);
        };
        self.buffer_id.get_mut_ref()
    }

    pub fn get_buffer_id(&self) -> u64 {
        self.buffer_id.unwrap_or(0)
    }

    pub fn clear_attach_session(&mut self) {
        self.attach_session = None;
    }

    pub fn has_attach_session(&self) -> bool {
        self.attach_session.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attach_session(&mut self, v: AttachSessionT) {
        self.attach_session = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_attach_session(&'a mut self) -> &'a mut AttachSessionT {
        if self.attach_session.is_none() {
            self.attach_session = Some(AttachSessionT::new());
        };
        self.attach_session.get_mut_ref()
    }

    pub fn get_attach_session(&'a self) -> &'a AttachSessionT {
        match self.attach_session {
            Some(ref v) => v,
            None => AttachSessionT::default_instance(),
        }
    }

    pub fn clear_connect(&mut self) {
        self.connect = None;
    }

    pub fn has_connect(&self) -> bool {
        self.connect.is_some()
    }

    // Param is passed by value, moved
    pub fn set_connect(&mut self, v: ConnectT) {
        self.connect = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_connect(&'a mut self) -> &'a mut ConnectT {
        if self.connect.is_none() {
            self.connect = Some(ConnectT::new());
        };
        self.connect.get_mut_ref()
    }

    pub fn get_connect(&'a self) -> &'a ConnectT {
        match self.connect {
            Some(ref v) => v,
            None => ConnectT::default_instance(),
        }
    }

    pub fn clear_register(&mut self) {
        self.register = None;
    }

    pub fn has_register(&self) -> bool {
        self.register.is_some()
    }

    // Param is passed by value, moved
    pub fn set_register(&mut self, v: RegisterT) {
        self.register = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_register(&'a mut self) -> &'a mut RegisterT {
        if self.register.is_none() {
            self.register = Some(RegisterT::new());
        };
        self.register.get_mut_ref()
    }

    pub fn get_register(&'a self) -> &'a RegisterT {
        match self.register {
            Some(ref v) => v,
            None => RegisterT::default_instance(),
        }
    }

    pub fn clear_join_channel(&mut self) {
        self.join_channel = None;
    }

    pub fn has_join_channel(&self) -> bool {
        self.join_channel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_join_channel(&mut self, v: JoinChannelT) {
        self.join_channel = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_join_channel(&'a mut self) -> &'a mut JoinChannelT {
        if self.join_channel.is_none() {
            self.join_channel = Some(JoinChannelT::new());
        };
        self.join_channel.get_mut_ref()
    }

    pub fn get_join_channel(&'a self) -> &'a JoinChannelT {
        match self.join_channel {
            Some(ref v) => v,
            None => JoinChannelT::default_instance(),
        }
    }

    pub fn clear_send_privmsg(&mut self) {
        self.send_privmsg = None;
    }

    pub fn has_send_privmsg(&self) -> bool {
        self.send_privmsg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_send_privmsg(&mut self, v: SendPrivmsgT) {
        self.send_privmsg = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_send_privmsg(&'a mut self) -> &'a mut SendPrivmsgT {
        if self.send_privmsg.is_none() {
            self.send_privmsg = Some(SendPrivmsgT::new());
        };
        self.send_privmsg.get_mut_ref()
    }

    pub fn get_send_privmsg(&'a self) -> &'a SendPrivmsgT {
        match self.send_privmsg {
            Some(ref v) => v,
            None => SendPrivmsgT::default_instance(),
        }
    }
}

impl Message for RemoteCommand {
    fn new() -> RemoteCommand {
        RemoteCommand::new()
    }

    fn clear(&mut self) {
        self.clear_packet_type();
        self.clear_network_id();
        self.clear_buffer_id();
        self.clear_attach_session();
        self.clear_connect();
        self.clear_register();
        self.clear_join_channel();
        self.clear_send_privmsg();
    }

    fn is_initialized(&self) -> bool {
        if self.packet_type.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = RemoteCommand_Type::new(is.read_int32());
                    self.packet_type = Some(tmp);
                },
                2 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_uint64();
                    self.network_id = Some(tmp);
                },
                3 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_uint64();
                    self.buffer_id = Some(tmp);
                },
                4 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = AttachSessionT::new();
                    is.merge_message(&mut tmp);
                    self.attach_session = Some(tmp);
                },
                5 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = ConnectT::new();
                    is.merge_message(&mut tmp);
                    self.connect = Some(tmp);
                },
                6 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = RegisterT::new();
                    is.merge_message(&mut tmp);
                    self.register = Some(tmp);
                },
                7 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = JoinChannelT::new();
                    is.merge_message(&mut tmp);
                    self.join_channel = Some(tmp);
                },
                8 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = SendPrivmsgT::new();
                    is.merge_message(&mut tmp);
                    self.send_privmsg = Some(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.packet_type.iter() {
            my_size += rt::enum_size(1, *value);
        };
        for value in self.network_id.iter() {
            my_size += rt::value_size(2, *value, wire_format::WireTypeVarint);
        };
        for value in self.buffer_id.iter() {
            my_size += rt::value_size(3, *value, wire_format::WireTypeVarint);
        };
        for value in self.attach_session.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.connect.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.register.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.join_channel.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.send_privmsg.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub enum RemoteCommand_Type {
    AttachSession = 1,
    GetNetworkList = 100,
    Connect = 200,
    Register = 201,
    JoinChannel = 202,
    SendPrivmsg = 203,
    GetBufferList = 204,
}

impl RemoteCommand_Type {
    pub fn new(value: i32) -> RemoteCommand_Type {
        match value {
            1 => AttachSession,
            100 => GetNetworkList,
            200 => Connect,
            201 => Register,
            202 => JoinChannel,
            203 => SendPrivmsg,
            204 => GetBufferList,
            _ => fail!()
        }
    }
}

impl ProtobufEnum for RemoteCommand_Type {
    fn value(&self) -> i32 {
        *self as i32
    }
}

#[deriving(Clone,Eq)]
pub struct AttachSessionT {
    session_id: Option<u64>,
}

impl<'a> AttachSessionT {
    pub fn new() -> AttachSessionT {
        AttachSessionT {
            session_id: None,
        }
    }

    pub fn default_instance() -> &'static AttachSessionT {
        static instance: AttachSessionT = AttachSessionT {
            session_id: None,
        };
        &'static instance
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.session_id {
            Some(ref v) => {
                os.write_uint64(1, *v);
            },
            None => {},
        };
    }

    pub fn clear_session_id(&mut self) {
        self.session_id = None;
    }

    pub fn has_session_id(&self) -> bool {
        self.session_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_session_id(&mut self, v: u64) {
        self.session_id = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_session_id(&'a mut self) -> &'a mut u64 {
        if self.session_id.is_none() {
            self.session_id = Some(0);
        };
        self.session_id.get_mut_ref()
    }

    pub fn get_session_id(&self) -> u64 {
        self.session_id.unwrap_or(0)
    }
}

impl Message for AttachSessionT {
    fn new() -> AttachSessionT {
        AttachSessionT::new()
    }

    fn clear(&mut self) {
        self.clear_session_id();
    }

    fn is_initialized(&self) -> bool {
        if self.session_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_uint64();
                    self.session_id = Some(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.session_id.iter() {
            my_size += rt::value_size(1, *value, wire_format::WireTypeVarint);
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub struct ConnectT {
    address: Option<~str>,
}

impl<'a> ConnectT {
    pub fn new() -> ConnectT {
        ConnectT {
            address: None,
        }
    }

    pub fn default_instance() -> &'static ConnectT {
        static instance: ConnectT = ConnectT {
            address: None,
        };
        &'static instance
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.address {
            Some(ref v) => {
                os.write_string(2, *v);
            },
            None => {},
        };
    }

    pub fn clear_address(&mut self) {
        self.address = None;
    }

    pub fn has_address(&self) -> bool {
        self.address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ~str) {
        self.address = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&'a mut self) -> &'a mut ~str {
        if self.address.is_none() {
            self.address = Some(~"");
        };
        self.address.get_mut_ref()
    }

    pub fn get_address(&'a self) -> &'a str {
        match self.address {
            Some(ref v) => v.as_slice(),
            None => &'a "",
        }
    }
}

impl Message for ConnectT {
    fn new() -> ConnectT {
        ConnectT::new()
    }

    fn clear(&mut self) {
        self.clear_address();
    }

    fn is_initialized(&self) -> bool {
        if self.address.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                2 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.address = Some(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.address.iter() {
            my_size += rt::string_size(2, *value);
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub struct RegisterT {
    nickname: Option<~str>,
}

impl<'a> RegisterT {
    pub fn new() -> RegisterT {
        RegisterT {
            nickname: None,
        }
    }

    pub fn default_instance() -> &'static RegisterT {
        static instance: RegisterT = RegisterT {
            nickname: None,
        };
        &'static instance
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.nickname {
            Some(ref v) => {
                os.write_string(2, *v);
            },
            None => {},
        };
    }

    pub fn clear_nickname(&mut self) {
        self.nickname = None;
    }

    pub fn has_nickname(&self) -> bool {
        self.nickname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nickname(&mut self, v: ~str) {
        self.nickname = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nickname(&'a mut self) -> &'a mut ~str {
        if self.nickname.is_none() {
            self.nickname = Some(~"");
        };
        self.nickname.get_mut_ref()
    }

    pub fn get_nickname(&'a self) -> &'a str {
        match self.nickname {
            Some(ref v) => v.as_slice(),
            None => &'a "",
        }
    }
}

impl Message for RegisterT {
    fn new() -> RegisterT {
        RegisterT::new()
    }

    fn clear(&mut self) {
        self.clear_nickname();
    }

    fn is_initialized(&self) -> bool {
        if self.nickname.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                2 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.nickname = Some(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.nickname.iter() {
            my_size += rt::string_size(2, *value);
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub struct JoinChannelT {
    channel: Option<~str>,
}

impl<'a> JoinChannelT {
    pub fn new() -> JoinChannelT {
        JoinChannelT {
            channel: None,
        }
    }

    pub fn default_instance() -> &'static JoinChannelT {
        static instance: JoinChannelT = JoinChannelT {
            channel: None,
        };
        &'static instance
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.channel {
            Some(ref v) => {
                os.write_string(2, *v);
            },
            None => {},
        };
    }

    pub fn clear_channel(&mut self) {
        self.channel = None;
    }

    pub fn has_channel(&self) -> bool {
        self.channel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel(&mut self, v: ~str) {
        self.channel = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_channel(&'a mut self) -> &'a mut ~str {
        if self.channel.is_none() {
            self.channel = Some(~"");
        };
        self.channel.get_mut_ref()
    }

    pub fn get_channel(&'a self) -> &'a str {
        match self.channel {
            Some(ref v) => v.as_slice(),
            None => &'a "",
        }
    }
}

impl Message for JoinChannelT {
    fn new() -> JoinChannelT {
        JoinChannelT::new()
    }

    fn clear(&mut self) {
        self.clear_channel();
    }

    fn is_initialized(&self) -> bool {
        if self.channel.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                2 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.channel = Some(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.channel.iter() {
            my_size += rt::string_size(2, *value);
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub struct SendPrivmsgT {
    target: Option<~str>,
    msg: Option<~str>,
}

impl<'a> SendPrivmsgT {
    pub fn new() -> SendPrivmsgT {
        SendPrivmsgT {
            target: None,
            msg: None,
        }
    }

    pub fn default_instance() -> &'static SendPrivmsgT {
        static instance: SendPrivmsgT = SendPrivmsgT {
            target: None,
            msg: None,
        };
        &'static instance
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.target {
            Some(ref v) => {
                os.write_string(2, *v);
            },
            None => {},
        };
        match self.msg {
            Some(ref v) => {
                os.write_string(3, *v);
            },
            None => {},
        };
    }

    pub fn clear_target(&mut self) {
        self.target = None;
    }

    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target(&mut self, v: ~str) {
        self.target = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_target(&'a mut self) -> &'a mut ~str {
        if self.target.is_none() {
            self.target = Some(~"");
        };
        self.target.get_mut_ref()
    }

    pub fn get_target(&'a self) -> &'a str {
        match self.target {
            Some(ref v) => v.as_slice(),
            None => &'a "",
        }
    }

    pub fn clear_msg(&mut self) {
        self.msg = None;
    }

    pub fn has_msg(&self) -> bool {
        self.msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ~str) {
        self.msg = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&'a mut self) -> &'a mut ~str {
        if self.msg.is_none() {
            self.msg = Some(~"");
        };
        self.msg.get_mut_ref()
    }

    pub fn get_msg(&'a self) -> &'a str {
        match self.msg {
            Some(ref v) => v.as_slice(),
            None => &'a "",
        }
    }
}

impl Message for SendPrivmsgT {
    fn new() -> SendPrivmsgT {
        SendPrivmsgT::new()
    }

    fn clear(&mut self) {
        self.clear_target();
        self.clear_msg();
    }

    fn is_initialized(&self) -> bool {
        if self.target.is_none() {
            return false;
        };
        if self.msg.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                2 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.target = Some(tmp);
                },
                3 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.msg = Some(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.target.iter() {
            my_size += rt::string_size(2, *value);
        };
        for value in self.msg.iter() {
            my_size += rt::string_size(3, *value);
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub struct RemoteMessage {
    packet_type: Option<RemoteMessage_Type>,
    network_id: Option<u64>,
    buffer_id: Option<u64>,
    message_id: Option<u64>,
    message_time: Option<u64>,
    network_list: ~[NetworkListT],
    disconnected: Option<DisconnectedT>,
    buffer_list: ~[BufferListT],
    new_buffer: Option<NewBufferT>,
    information: Option<InformationT>,
}

impl<'a> RemoteMessage {
    pub fn new() -> RemoteMessage {
        RemoteMessage {
            packet_type: None,
            network_id: None,
            buffer_id: None,
            message_id: None,
            message_time: None,
            network_list: ~[],
            disconnected: None,
            buffer_list: ~[],
            new_buffer: None,
            information: None,
        }
    }

    pub fn default_instance() -> &'static RemoteMessage {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: RemoteMessage = RemoteMessage {
//             packet_type: None,
//             network_id: None,
//             buffer_id: None,
//             message_id: None,
//             message_time: None,
//             network_list: ~[],
//             disconnected: None,
//             buffer_list: ~[],
//             new_buffer: None,
//             information: None,
//         };
//         &'static instance
        fail!("TODO");
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.packet_type {
            Some(ref v) => {
                os.write_enum(1, *v as i32);
            },
            None => {},
        };
        match self.network_id {
            Some(ref v) => {
                os.write_uint64(2, *v);
            },
            None => {},
        };
        match self.buffer_id {
            Some(ref v) => {
                os.write_uint64(3, *v);
            },
            None => {},
        };
        match self.message_id {
            Some(ref v) => {
                os.write_uint64(4, *v);
            },
            None => {},
        };
        match self.message_time {
            Some(ref v) => {
                os.write_uint64(5, *v);
            },
            None => {},
        };
        for v in self.network_list.iter() {
            os.write_tag(6, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
        match self.disconnected {
            Some(ref v) => {
                os.write_tag(8, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
        for v in self.buffer_list.iter() {
            os.write_tag(9, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
        match self.new_buffer {
            Some(ref v) => {
                os.write_tag(10, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
        match self.information {
            Some(ref v) => {
                os.write_tag(11, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
    }

    pub fn clear_packet_type(&mut self) {
        self.packet_type = None;
    }

    pub fn has_packet_type(&self) -> bool {
        self.packet_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packet_type(&mut self, v: RemoteMessage_Type) {
        self.packet_type = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_packet_type(&'a mut self) -> &'a mut RemoteMessage_Type {
        if self.packet_type.is_none() {
            self.packet_type = Some(RemoteMessage_Type::new(0));
        };
        self.packet_type.get_mut_ref()
    }

    pub fn get_packet_type(&self) -> RemoteMessage_Type {
        self.packet_type.unwrap_or(RemoteMessage_Type::new(0))
    }

    pub fn clear_network_id(&mut self) {
        self.network_id = None;
    }

    pub fn has_network_id(&self) -> bool {
        self.network_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_network_id(&mut self, v: u64) {
        self.network_id = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_network_id(&'a mut self) -> &'a mut u64 {
        if self.network_id.is_none() {
            self.network_id = Some(0);
        };
        self.network_id.get_mut_ref()
    }

    pub fn get_network_id(&self) -> u64 {
        self.network_id.unwrap_or(0)
    }

    pub fn clear_buffer_id(&mut self) {
        self.buffer_id = None;
    }

    pub fn has_buffer_id(&self) -> bool {
        self.buffer_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_buffer_id(&mut self, v: u64) {
        self.buffer_id = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_buffer_id(&'a mut self) -> &'a mut u64 {
        if self.buffer_id.is_none() {
            self.buffer_id = Some(0);
        };
        self.buffer_id.get_mut_ref()
    }

    pub fn get_buffer_id(&self) -> u64 {
        self.buffer_id.unwrap_or(0)
    }

    pub fn clear_message_id(&mut self) {
        self.message_id = None;
    }

    pub fn has_message_id(&self) -> bool {
        self.message_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message_id(&mut self, v: u64) {
        self.message_id = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message_id(&'a mut self) -> &'a mut u64 {
        if self.message_id.is_none() {
            self.message_id = Some(0);
        };
        self.message_id.get_mut_ref()
    }

    pub fn get_message_id(&self) -> u64 {
        self.message_id.unwrap_or(0)
    }

    pub fn clear_message_time(&mut self) {
        self.message_time = None;
    }

    pub fn has_message_time(&self) -> bool {
        self.message_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message_time(&mut self, v: u64) {
        self.message_time = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message_time(&'a mut self) -> &'a mut u64 {
        if self.message_time.is_none() {
            self.message_time = Some(0);
        };
        self.message_time.get_mut_ref()
    }

    pub fn get_message_time(&self) -> u64 {
        self.message_time.unwrap_or(0)
    }

    pub fn clear_network_list(&mut self) {
        self.network_list.clear();
    }

    // Param is passed by value, moved
    pub fn set_network_list(&mut self, v: ~[NetworkListT]) {
        self.network_list = v;
    }

    // Mutable pointer to the field.
    pub fn mut_network_list(&'a mut self) -> &'a mut ~[NetworkListT] {
        &mut self.network_list
    }

    pub fn get_network_list(&'a self) -> &'a [NetworkListT] {
        rt::as_slice_tmp(&self.network_list)
    }

    pub fn add_network_list(&mut self, v: NetworkListT) {
        self.network_list.push(v);
    }

    pub fn clear_disconnected(&mut self) {
        self.disconnected = None;
    }

    pub fn has_disconnected(&self) -> bool {
        self.disconnected.is_some()
    }

    // Param is passed by value, moved
    pub fn set_disconnected(&mut self, v: DisconnectedT) {
        self.disconnected = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_disconnected(&'a mut self) -> &'a mut DisconnectedT {
        if self.disconnected.is_none() {
            self.disconnected = Some(DisconnectedT::new());
        };
        self.disconnected.get_mut_ref()
    }

    pub fn get_disconnected(&'a self) -> &'a DisconnectedT {
        match self.disconnected {
            Some(ref v) => v,
            None => DisconnectedT::default_instance(),
        }
    }

    pub fn clear_buffer_list(&mut self) {
        self.buffer_list.clear();
    }

    // Param is passed by value, moved
    pub fn set_buffer_list(&mut self, v: ~[BufferListT]) {
        self.buffer_list = v;
    }

    // Mutable pointer to the field.
    pub fn mut_buffer_list(&'a mut self) -> &'a mut ~[BufferListT] {
        &mut self.buffer_list
    }

    pub fn get_buffer_list(&'a self) -> &'a [BufferListT] {
        rt::as_slice_tmp(&self.buffer_list)
    }

    pub fn add_buffer_list(&mut self, v: BufferListT) {
        self.buffer_list.push(v);
    }

    pub fn clear_new_buffer(&mut self) {
        self.new_buffer = None;
    }

    pub fn has_new_buffer(&self) -> bool {
        self.new_buffer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_buffer(&mut self, v: NewBufferT) {
        self.new_buffer = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_new_buffer(&'a mut self) -> &'a mut NewBufferT {
        if self.new_buffer.is_none() {
            self.new_buffer = Some(NewBufferT::new());
        };
        self.new_buffer.get_mut_ref()
    }

    pub fn get_new_buffer(&'a self) -> &'a NewBufferT {
        match self.new_buffer {
            Some(ref v) => v,
            None => NewBufferT::default_instance(),
        }
    }

    pub fn clear_information(&mut self) {
        self.information = None;
    }

    pub fn has_information(&self) -> bool {
        self.information.is_some()
    }

    // Param is passed by value, moved
    pub fn set_information(&mut self, v: InformationT) {
        self.information = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_information(&'a mut self) -> &'a mut InformationT {
        if self.information.is_none() {
            self.information = Some(InformationT::new());
        };
        self.information.get_mut_ref()
    }

    pub fn get_information(&'a self) -> &'a InformationT {
        match self.information {
            Some(ref v) => v,
            None => InformationT::default_instance(),
        }
    }
}

impl Message for RemoteMessage {
    fn new() -> RemoteMessage {
        RemoteMessage::new()
    }

    fn clear(&mut self) {
        self.clear_packet_type();
        self.clear_network_id();
        self.clear_buffer_id();
        self.clear_message_id();
        self.clear_message_time();
        self.clear_network_list();
        self.clear_disconnected();
        self.clear_buffer_list();
        self.clear_new_buffer();
        self.clear_information();
    }

    fn is_initialized(&self) -> bool {
        if self.packet_type.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = RemoteMessage_Type::new(is.read_int32());
                    self.packet_type = Some(tmp);
                },
                2 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_uint64();
                    self.network_id = Some(tmp);
                },
                3 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_uint64();
                    self.buffer_id = Some(tmp);
                },
                4 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_uint64();
                    self.message_id = Some(tmp);
                },
                5 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_uint64();
                    self.message_time = Some(tmp);
                },
                6 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = NetworkListT::new();
                    is.merge_message(&mut tmp);
                    self.network_list.push(tmp);
                },
                8 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = DisconnectedT::new();
                    is.merge_message(&mut tmp);
                    self.disconnected = Some(tmp);
                },
                9 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = BufferListT::new();
                    is.merge_message(&mut tmp);
                    self.buffer_list.push(tmp);
                },
                10 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = NewBufferT::new();
                    is.merge_message(&mut tmp);
                    self.new_buffer = Some(tmp);
                },
                11 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = InformationT::new();
                    is.merge_message(&mut tmp);
                    self.information = Some(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.packet_type.iter() {
            my_size += rt::enum_size(1, *value);
        };
        for value in self.network_id.iter() {
            my_size += rt::value_size(2, *value, wire_format::WireTypeVarint);
        };
        for value in self.buffer_id.iter() {
            my_size += rt::value_size(3, *value, wire_format::WireTypeVarint);
        };
        for value in self.message_id.iter() {
            my_size += rt::value_size(4, *value, wire_format::WireTypeVarint);
        };
        for value in self.message_time.iter() {
            my_size += rt::value_size(5, *value, wire_format::WireTypeVarint);
        };
        for value in self.network_list.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.disconnected.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.buffer_list.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.new_buffer.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.information.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub enum RemoteMessage_Type {
    NetworkList = 100,
    Connected = 201,
    Disconnected = 202,
    BufferList = 203,
    NewBuffer = 204,
    Information = 205,
}

impl RemoteMessage_Type {
    pub fn new(value: i32) -> RemoteMessage_Type {
        match value {
            100 => NetworkList,
            201 => Connected,
            202 => Disconnected,
            203 => BufferList,
            204 => NewBuffer,
            205 => Information,
            _ => fail!()
        }
    }
}

impl ProtobufEnum for RemoteMessage_Type {
    fn value(&self) -> i32 {
        *self as i32
    }
}

#[deriving(Clone,Eq)]
pub struct NetworkListT {
    id: Option<u64>,
}

impl<'a> NetworkListT {
    pub fn new() -> NetworkListT {
        NetworkListT {
            id: None,
        }
    }

    pub fn default_instance() -> &'static NetworkListT {
        static instance: NetworkListT = NetworkListT {
            id: None,
        };
        &'static instance
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.id {
            Some(ref v) => {
                os.write_uint64(1, *v);
            },
            None => {},
        };
    }

    pub fn clear_id(&mut self) {
        self.id = None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&'a mut self) -> &'a mut u64 {
        if self.id.is_none() {
            self.id = Some(0);
        };
        self.id.get_mut_ref()
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }
}

impl Message for NetworkListT {
    fn new() -> NetworkListT {
        NetworkListT::new()
    }

    fn clear(&mut self) {
        self.clear_id();
    }

    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_uint64();
                    self.id = Some(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.id.iter() {
            my_size += rt::value_size(1, *value, wire_format::WireTypeVarint);
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub struct DisconnectedT {
    reason: Option<~str>,
}

impl<'a> DisconnectedT {
    pub fn new() -> DisconnectedT {
        DisconnectedT {
            reason: None,
        }
    }

    pub fn default_instance() -> &'static DisconnectedT {
        static instance: DisconnectedT = DisconnectedT {
            reason: None,
        };
        &'static instance
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.reason {
            Some(ref v) => {
                os.write_string(1, *v);
            },
            None => {},
        };
    }

    pub fn clear_reason(&mut self) {
        self.reason = None;
    }

    pub fn has_reason(&self) -> bool {
        self.reason.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reason(&mut self, v: ~str) {
        self.reason = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reason(&'a mut self) -> &'a mut ~str {
        if self.reason.is_none() {
            self.reason = Some(~"");
        };
        self.reason.get_mut_ref()
    }

    pub fn get_reason(&'a self) -> &'a str {
        match self.reason {
            Some(ref v) => v.as_slice(),
            None => &'a "",
        }
    }
}

impl Message for DisconnectedT {
    fn new() -> DisconnectedT {
        DisconnectedT::new()
    }

    fn clear(&mut self) {
        self.clear_reason();
    }

    fn is_initialized(&self) -> bool {
        if self.reason.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.reason = Some(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.reason.iter() {
            my_size += rt::string_size(1, *value);
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub struct BufferRole {
    buffer_type: Option<BufferRole_Type>,
    name: Option<~str>,
}

impl<'a> BufferRole {
    pub fn new() -> BufferRole {
        BufferRole {
            buffer_type: None,
            name: None,
        }
    }

    pub fn default_instance() -> &'static BufferRole {
        static instance: BufferRole = BufferRole {
            buffer_type: None,
            name: None,
        };
        &'static instance
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.buffer_type {
            Some(ref v) => {
                os.write_enum(1, *v as i32);
            },
            None => {},
        };
        match self.name {
            Some(ref v) => {
                os.write_string(2, *v);
            },
            None => {},
        };
    }

    pub fn clear_buffer_type(&mut self) {
        self.buffer_type = None;
    }

    pub fn has_buffer_type(&self) -> bool {
        self.buffer_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_buffer_type(&mut self, v: BufferRole_Type) {
        self.buffer_type = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_buffer_type(&'a mut self) -> &'a mut BufferRole_Type {
        if self.buffer_type.is_none() {
            self.buffer_type = Some(BufferRole_Type::new(0));
        };
        self.buffer_type.get_mut_ref()
    }

    pub fn get_buffer_type(&self) -> BufferRole_Type {
        self.buffer_type.unwrap_or(BufferRole_Type::new(0))
    }

    pub fn clear_name(&mut self) {
        self.name = None;
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ~str) {
        self.name = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&'a mut self) -> &'a mut ~str {
        if self.name.is_none() {
            self.name = Some(~"");
        };
        self.name.get_mut_ref()
    }

    pub fn get_name(&'a self) -> &'a str {
        match self.name {
            Some(ref v) => v.as_slice(),
            None => &'a "",
        }
    }
}

impl Message for BufferRole {
    fn new() -> BufferRole {
        BufferRole::new()
    }

    fn clear(&mut self) {
        self.clear_buffer_type();
        self.clear_name();
    }

    fn is_initialized(&self) -> bool {
        if self.buffer_type.is_none() {
            return false;
        };
        if self.name.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = BufferRole_Type::new(is.read_int32());
                    self.buffer_type = Some(tmp);
                },
                2 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.name = Some(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.buffer_type.iter() {
            my_size += rt::enum_size(1, *value);
        };
        for value in self.name.iter() {
            my_size += rt::string_size(2, *value);
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub enum BufferRole_Type {
    Status = 1,
    Channel = 2,
    Query = 3,
}

impl BufferRole_Type {
    pub fn new(value: i32) -> BufferRole_Type {
        match value {
            1 => Status,
            2 => Channel,
            3 => Query,
            _ => fail!()
        }
    }
}

impl ProtobufEnum for BufferRole_Type {
    fn value(&self) -> i32 {
        *self as i32
    }
}

#[deriving(Clone,Eq)]
pub struct BufferListT {
    id: Option<u64>,
    role: Option<BufferRole>,
}

impl<'a> BufferListT {
    pub fn new() -> BufferListT {
        BufferListT {
            id: None,
            role: None,
        }
    }

    pub fn default_instance() -> &'static BufferListT {
        static instance: BufferListT = BufferListT {
            id: None,
            role: None,
        };
        &'static instance
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.id {
            Some(ref v) => {
                os.write_uint64(1, *v);
            },
            None => {},
        };
        match self.role {
            Some(ref v) => {
                os.write_tag(2, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
    }

    pub fn clear_id(&mut self) {
        self.id = None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&'a mut self) -> &'a mut u64 {
        if self.id.is_none() {
            self.id = Some(0);
        };
        self.id.get_mut_ref()
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    pub fn clear_role(&mut self) {
        self.role = None;
    }

    pub fn has_role(&self) -> bool {
        self.role.is_some()
    }

    // Param is passed by value, moved
    pub fn set_role(&mut self, v: BufferRole) {
        self.role = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_role(&'a mut self) -> &'a mut BufferRole {
        if self.role.is_none() {
            self.role = Some(BufferRole::new());
        };
        self.role.get_mut_ref()
    }

    pub fn get_role(&'a self) -> &'a BufferRole {
        match self.role {
            Some(ref v) => v,
            None => BufferRole::default_instance(),
        }
    }
}

impl Message for BufferListT {
    fn new() -> BufferListT {
        BufferListT::new()
    }

    fn clear(&mut self) {
        self.clear_id();
        self.clear_role();
    }

    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        if self.role.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_uint64();
                    self.id = Some(tmp);
                },
                2 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = BufferRole::new();
                    is.merge_message(&mut tmp);
                    self.role = Some(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.id.iter() {
            my_size += rt::value_size(1, *value, wire_format::WireTypeVarint);
        };
        for value in self.role.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub struct NewBufferT {
    id: Option<u64>,
    role: Option<BufferRole>,
}

impl<'a> NewBufferT {
    pub fn new() -> NewBufferT {
        NewBufferT {
            id: None,
            role: None,
        }
    }

    pub fn default_instance() -> &'static NewBufferT {
        static instance: NewBufferT = NewBufferT {
            id: None,
            role: None,
        };
        &'static instance
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.id {
            Some(ref v) => {
                os.write_uint64(1, *v);
            },
            None => {},
        };
        match self.role {
            Some(ref v) => {
                os.write_tag(2, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
    }

    pub fn clear_id(&mut self) {
        self.id = None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&'a mut self) -> &'a mut u64 {
        if self.id.is_none() {
            self.id = Some(0);
        };
        self.id.get_mut_ref()
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    pub fn clear_role(&mut self) {
        self.role = None;
    }

    pub fn has_role(&self) -> bool {
        self.role.is_some()
    }

    // Param is passed by value, moved
    pub fn set_role(&mut self, v: BufferRole) {
        self.role = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_role(&'a mut self) -> &'a mut BufferRole {
        if self.role.is_none() {
            self.role = Some(BufferRole::new());
        };
        self.role.get_mut_ref()
    }

    pub fn get_role(&'a self) -> &'a BufferRole {
        match self.role {
            Some(ref v) => v,
            None => BufferRole::default_instance(),
        }
    }
}

impl Message for NewBufferT {
    fn new() -> NewBufferT {
        NewBufferT::new()
    }

    fn clear(&mut self) {
        self.clear_id();
        self.clear_role();
    }

    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        if self.role.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_uint64();
                    self.id = Some(tmp);
                },
                2 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = BufferRole::new();
                    is.merge_message(&mut tmp);
                    self.role = Some(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.id.iter() {
            my_size += rt::value_size(1, *value, wire_format::WireTypeVarint);
        };
        for value in self.role.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub struct InformationT {
    msg: Option<~str>,
}

impl<'a> InformationT {
    pub fn new() -> InformationT {
        InformationT {
            msg: None,
        }
    }

    pub fn default_instance() -> &'static InformationT {
        static instance: InformationT = InformationT {
            msg: None,
        };
        &'static instance
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.msg {
            Some(ref v) => {
                os.write_string(1, *v);
            },
            None => {},
        };
    }

    pub fn clear_msg(&mut self) {
        self.msg = None;
    }

    pub fn has_msg(&self) -> bool {
        self.msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ~str) {
        self.msg = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&'a mut self) -> &'a mut ~str {
        if self.msg.is_none() {
            self.msg = Some(~"");
        };
        self.msg.get_mut_ref()
    }

    pub fn get_msg(&'a self) -> &'a str {
        match self.msg {
            Some(ref v) => v.as_slice(),
            None => &'a "",
        }
    }
}

impl Message for InformationT {
    fn new() -> InformationT {
        InformationT::new()
    }

    fn clear(&mut self) {
        self.clear_msg();
    }

    fn is_initialized(&self) -> bool {
        if self.msg.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.msg = Some(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.msg.iter() {
            my_size += rt::string_size(1, *value);
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}
