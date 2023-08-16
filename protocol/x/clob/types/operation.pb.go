// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: dydxprotocol/clob/operation.proto

package types

import (
	fmt "fmt"
	proto "github.com/cosmos/gogoproto/proto"
	io "io"
	math "math"
	math_bits "math/bits"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
// A compilation error at this line likely means your copy of the
// proto package needs to be updated.
const _ = proto.GoGoProtoPackageIsVersion3 // please upgrade the proto package

// Operation represents an operation in the proposed operations. Operation is
// used internally within the memclob only.
type Operation struct {
	// operation represents the operation that occurred, which can be a match,
	// short term order placement, short term order cancellation, or the placement
	// of a pre-existing stateful order.
	//
	// Types that are valid to be assigned to Operation:
	//	*Operation_Match
	//	*Operation_ShortTermOrderPlacement
	//	*Operation_ShortTermOrderCancellation
	//	*Operation_PreexistingStatefulOrder
	Operation isOperation_Operation `protobuf_oneof:"operation"`
}

func (m *Operation) Reset()         { *m = Operation{} }
func (m *Operation) String() string { return proto.CompactTextString(m) }
func (*Operation) ProtoMessage()    {}
func (*Operation) Descriptor() ([]byte, []int) {
	return fileDescriptor_5906bab2b2e9b3cf, []int{0}
}
func (m *Operation) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *Operation) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_Operation.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *Operation) XXX_Merge(src proto.Message) {
	xxx_messageInfo_Operation.Merge(m, src)
}
func (m *Operation) XXX_Size() int {
	return m.Size()
}
func (m *Operation) XXX_DiscardUnknown() {
	xxx_messageInfo_Operation.DiscardUnknown(m)
}

var xxx_messageInfo_Operation proto.InternalMessageInfo

type isOperation_Operation interface {
	isOperation_Operation()
	MarshalTo([]byte) (int, error)
	Size() int
}

type Operation_Match struct {
	Match *ClobMatch `protobuf:"bytes,1,opt,name=match,proto3,oneof" json:"match,omitempty"`
}
type Operation_ShortTermOrderPlacement struct {
	ShortTermOrderPlacement *MsgPlaceOrder `protobuf:"bytes,2,opt,name=short_term_order_placement,json=shortTermOrderPlacement,proto3,oneof" json:"short_term_order_placement,omitempty"`
}
type Operation_ShortTermOrderCancellation struct {
	ShortTermOrderCancellation *MsgCancelOrder `protobuf:"bytes,3,opt,name=short_term_order_cancellation,json=shortTermOrderCancellation,proto3,oneof" json:"short_term_order_cancellation,omitempty"`
}
type Operation_PreexistingStatefulOrder struct {
	PreexistingStatefulOrder *OrderId `protobuf:"bytes,4,opt,name=preexisting_stateful_order,json=preexistingStatefulOrder,proto3,oneof" json:"preexisting_stateful_order,omitempty"`
}

func (*Operation_Match) isOperation_Operation()                      {}
func (*Operation_ShortTermOrderPlacement) isOperation_Operation()    {}
func (*Operation_ShortTermOrderCancellation) isOperation_Operation() {}
func (*Operation_PreexistingStatefulOrder) isOperation_Operation()   {}

func (m *Operation) GetOperation() isOperation_Operation {
	if m != nil {
		return m.Operation
	}
	return nil
}

func (m *Operation) GetMatch() *ClobMatch {
	if x, ok := m.GetOperation().(*Operation_Match); ok {
		return x.Match
	}
	return nil
}

func (m *Operation) GetShortTermOrderPlacement() *MsgPlaceOrder {
	if x, ok := m.GetOperation().(*Operation_ShortTermOrderPlacement); ok {
		return x.ShortTermOrderPlacement
	}
	return nil
}

func (m *Operation) GetShortTermOrderCancellation() *MsgCancelOrder {
	if x, ok := m.GetOperation().(*Operation_ShortTermOrderCancellation); ok {
		return x.ShortTermOrderCancellation
	}
	return nil
}

func (m *Operation) GetPreexistingStatefulOrder() *OrderId {
	if x, ok := m.GetOperation().(*Operation_PreexistingStatefulOrder); ok {
		return x.PreexistingStatefulOrder
	}
	return nil
}

// XXX_OneofWrappers is for the internal use of the proto package.
func (*Operation) XXX_OneofWrappers() []interface{} {
	return []interface{}{
		(*Operation_Match)(nil),
		(*Operation_ShortTermOrderPlacement)(nil),
		(*Operation_ShortTermOrderCancellation)(nil),
		(*Operation_PreexistingStatefulOrder)(nil),
	}
}

// InternalOperation represents an internal operation in the operations to
// propose. InternalOperation is used internally within the memclob only.
type InternalOperation struct {
	// operation represents the operation that occurred, which can be a match,
	// Short-Term order placement, or the placement of a pre-existing stateful
	// order.
	//
	// Types that are valid to be assigned to Operation:
	//
	//	*InternalOperation_Match
	//	*InternalOperation_ShortTermOrderPlacement
	//	*InternalOperation_PreexistingStatefulOrder
	//	*InternalOperation_OrderRemoval
	Operation isInternalOperation_Operation `protobuf_oneof:"operation"`
}

func (m *InternalOperation) Reset()         { *m = InternalOperation{} }
func (m *InternalOperation) String() string { return proto.CompactTextString(m) }
func (*InternalOperation) ProtoMessage()    {}
func (*InternalOperation) Descriptor() ([]byte, []int) {
	return fileDescriptor_5906bab2b2e9b3cf, []int{1}
}
func (m *InternalOperation) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *InternalOperation) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_InternalOperation.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *InternalOperation) XXX_Merge(src proto.Message) {
	xxx_messageInfo_InternalOperation.Merge(m, src)
}
func (m *InternalOperation) XXX_Size() int {
	return m.Size()
}
func (m *InternalOperation) XXX_DiscardUnknown() {
	xxx_messageInfo_InternalOperation.DiscardUnknown(m)
}

var xxx_messageInfo_InternalOperation proto.InternalMessageInfo

type isInternalOperation_Operation interface {
	isInternalOperation_Operation()
	MarshalTo([]byte) (int, error)
	Size() int
}

type InternalOperation_Match struct {
	Match *ClobMatch `protobuf:"bytes,1,opt,name=match,proto3,oneof" json:"match,omitempty"`
}
type InternalOperation_ShortTermOrderPlacement struct {
	ShortTermOrderPlacement *MsgPlaceOrder `protobuf:"bytes,2,opt,name=short_term_order_placement,json=shortTermOrderPlacement,proto3,oneof" json:"short_term_order_placement,omitempty"`
}
type InternalOperation_PreexistingStatefulOrder struct {
	PreexistingStatefulOrder *OrderId `protobuf:"bytes,3,opt,name=preexisting_stateful_order,json=preexistingStatefulOrder,proto3,oneof" json:"preexisting_stateful_order,omitempty"`
}
type InternalOperation_OrderRemoval struct {
	OrderRemoval *OrderRemoval `protobuf:"bytes,4,opt,name=order_removal,json=orderRemoval,proto3,oneof" json:"order_removal,omitempty"`
}

func (*InternalOperation_Match) isInternalOperation_Operation()                    {}
func (*InternalOperation_ShortTermOrderPlacement) isInternalOperation_Operation()  {}
func (*InternalOperation_PreexistingStatefulOrder) isInternalOperation_Operation() {}
func (*InternalOperation_OrderRemoval) isInternalOperation_Operation()             {}

func (m *InternalOperation) GetOperation() isInternalOperation_Operation {
	if m != nil {
		return m.Operation
	}
	return nil
}

func (m *InternalOperation) GetMatch() *ClobMatch {
	if x, ok := m.GetOperation().(*InternalOperation_Match); ok {
		return x.Match
	}
	return nil
}

func (m *InternalOperation) GetShortTermOrderPlacement() *MsgPlaceOrder {
	if x, ok := m.GetOperation().(*InternalOperation_ShortTermOrderPlacement); ok {
		return x.ShortTermOrderPlacement
	}
	return nil
}

func (m *InternalOperation) GetPreexistingStatefulOrder() *OrderId {
	if x, ok := m.GetOperation().(*InternalOperation_PreexistingStatefulOrder); ok {
		return x.PreexistingStatefulOrder
	}
	return nil
}

func (m *InternalOperation) GetOrderRemoval() *OrderRemoval {
	if x, ok := m.GetOperation().(*InternalOperation_OrderRemoval); ok {
		return x.OrderRemoval
	}
	return nil
}

// XXX_OneofWrappers is for the internal use of the proto package.
func (*InternalOperation) XXX_OneofWrappers() []interface{} {
	return []interface{}{
		(*InternalOperation_Match)(nil),
		(*InternalOperation_ShortTermOrderPlacement)(nil),
		(*InternalOperation_PreexistingStatefulOrder)(nil),
		(*InternalOperation_OrderRemoval)(nil),
	}
}

func init() {
	proto.RegisterType((*Operation)(nil), "dydxprotocol.clob.Operation")
	proto.RegisterType((*InternalOperation)(nil), "dydxprotocol.clob.InternalOperation")
}

func init() { proto.RegisterFile("dydxprotocol/clob/operation.proto", fileDescriptor_5906bab2b2e9b3cf) }

var fileDescriptor_5906bab2b2e9b3cf = []byte{
	// 388 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0xcc, 0x53, 0xcd, 0x6e, 0xa2, 0x40,
	0x00, 0x06, 0xdd, 0xdd, 0xc4, 0x71, 0xf7, 0x20, 0x97, 0x25, 0xa4, 0xe2, 0xcf, 0xa1, 0xed, 0x09,
	0x92, 0xd6, 0x17, 0xa8, 0x26, 0x0d, 0x1e, 0x8c, 0x86, 0xf6, 0xe4, 0x85, 0xc0, 0x30, 0x2a, 0xc9,
	0xc0, 0x90, 0x61, 0x34, 0xf8, 0x16, 0x7d, 0xac, 0x1e, 0x3d, 0x36, 0xe9, 0xa5, 0xd1, 0x47, 0xe8,
	0x0b, 0x34, 0x0c, 0xd0, 0x40, 0xc0, 0x5e, 0x7a, 0xe9, 0x71, 0xe6, 0xfb, 0xcd, 0x97, 0x19, 0x30,
	0x70, 0xf7, 0x6e, 0x1c, 0x52, 0xc2, 0x08, 0x24, 0x58, 0x87, 0x98, 0x38, 0x3a, 0x09, 0x11, 0xb5,
	0x99, 0x47, 0x02, 0x8d, 0xdf, 0x4b, 0x9d, 0x22, 0x45, 0x4b, 0x28, 0x4a, 0xaf, 0xaa, 0xf2, 0x6d,
	0x06, 0x37, 0x28, 0x4a, 0x35, 0x4a, 0xb7, 0xc6, 0x96, 0xba, 0x88, 0x66, 0xf0, 0xe5, 0x19, 0xd8,
	0xa2, 0xc8, 0x27, 0x3b, 0x1b, 0xe7, 0x36, 0x4a, 0x95, 0xc7, 0xe2, 0x14, 0x1b, 0xbe, 0x37, 0x40,
	0x6b, 0x9e, 0x57, 0x95, 0x46, 0xe0, 0x37, 0x6f, 0x20, 0x8b, 0x7d, 0xf1, 0xba, 0x7d, 0x73, 0xa1,
	0x55, 0x4a, 0x6b, 0x13, 0x4c, 0x9c, 0x59, 0xc2, 0x31, 0x04, 0x33, 0x25, 0x4b, 0x16, 0x50, 0xa2,
	0x0d, 0xa1, 0xcc, 0x62, 0x88, 0xfa, 0x56, 0x5a, 0x21, 0xc4, 0x36, 0x44, 0x3e, 0x0a, 0x98, 0xdc,
	0xe0, 0x56, 0xfd, 0x1a, 0xab, 0x59, 0xb4, 0x5e, 0x24, 0xb4, 0x79, 0xa2, 0x30, 0x04, 0xf3, 0x3f,
	0x77, 0x79, 0x44, 0xd4, 0xe7, 0x37, 0x8b, 0xdc, 0x42, 0x5a, 0x81, 0x6e, 0x25, 0x00, 0xda, 0x01,
	0x44, 0x18, 0xf3, 0xde, 0x72, 0x93, 0x67, 0x0c, 0xea, 0x33, 0x26, 0x9c, 0x99, 0x87, 0x28, 0xe5,
	0x90, 0x49, 0xc1, 0x46, 0x5a, 0x02, 0x25, 0xa4, 0x08, 0xc5, 0x5e, 0xc4, 0xbc, 0x60, 0x6d, 0x45,
	0xcc, 0x66, 0x68, 0xb5, 0xc5, 0x69, 0xa2, 0xfc, 0x8b, 0x87, 0x28, 0x35, 0x21, 0xdc, 0x69, 0xea,
	0x1a, 0x82, 0x29, 0x17, 0xf4, 0x0f, 0x99, 0x9c, 0xa3, 0xe3, 0x36, 0x68, 0x7d, 0x3e, 0x89, 0xe1,
	0x6b, 0x03, 0x74, 0xa6, 0x01, 0x43, 0x34, 0xb0, 0xf1, 0x8f, 0x5f, 0xff, 0xeb, 0x55, 0x9a, 0xdf,
	0x59, 0x45, 0xba, 0x07, 0xff, 0x4a, 0x4f, 0x36, 0x1b, 0xb9, 0x77, 0xce, 0xce, 0x4c, 0x69, 0x86,
	0x60, 0xfe, 0x25, 0x85, 0x73, 0x69, 0xdd, 0xf1, 0xdd, 0xf3, 0x51, 0x15, 0x0f, 0x47, 0x55, 0x7c,
	0x3b, 0xaa, 0xe2, 0xd3, 0x49, 0x15, 0x0e, 0x27, 0x55, 0x78, 0x39, 0xa9, 0xc2, 0xf2, 0x6a, 0xed,
	0xb1, 0xcd, 0xd6, 0xd1, 0x20, 0xf1, 0xf5, 0xd2, 0xa7, 0xd8, 0x8d, 0xf4, 0x38, 0xfb, 0x19, 0xfb,
	0x10, 0x45, 0xce, 0x1f, 0x8e, 0xdc, 0x7e, 0x04, 0x00, 0x00, 0xff, 0xff, 0x51, 0xa4, 0x90, 0x46,
	0xd9, 0x03, 0x00, 0x00,
}

func (m *Operation) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *Operation) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *Operation) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if m.Operation != nil {
		{
			size := m.Operation.Size()
			i -= size
			if _, err := m.Operation.MarshalTo(dAtA[i:]); err != nil {
				return 0, err
			}
		}
	}
	return len(dAtA) - i, nil
}

func (m *Operation_Match) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *Operation_Match) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	if m.Match != nil {
		{
			size, err := m.Match.MarshalToSizedBuffer(dAtA[:i])
			if err != nil {
				return 0, err
			}
			i -= size
			i = encodeVarintOperation(dAtA, i, uint64(size))
		}
		i--
		dAtA[i] = 0xa
	}
	return len(dAtA) - i, nil
}
func (m *Operation_ShortTermOrderPlacement) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *Operation_ShortTermOrderPlacement) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	if m.ShortTermOrderPlacement != nil {
		{
			size, err := m.ShortTermOrderPlacement.MarshalToSizedBuffer(dAtA[:i])
			if err != nil {
				return 0, err
			}
			i -= size
			i = encodeVarintOperation(dAtA, i, uint64(size))
		}
		i--
		dAtA[i] = 0x12
	}
	return len(dAtA) - i, nil
}
func (m *Operation_ShortTermOrderCancellation) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *Operation_ShortTermOrderCancellation) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	if m.ShortTermOrderCancellation != nil {
		{
			size, err := m.ShortTermOrderCancellation.MarshalToSizedBuffer(dAtA[:i])
			if err != nil {
				return 0, err
			}
			i -= size
			i = encodeVarintOperation(dAtA, i, uint64(size))
		}
		i--
		dAtA[i] = 0x1a
	}
	return len(dAtA) - i, nil
}
func (m *Operation_PreexistingStatefulOrder) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *Operation_PreexistingStatefulOrder) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	if m.PreexistingStatefulOrder != nil {
		{
			size, err := m.PreexistingStatefulOrder.MarshalToSizedBuffer(dAtA[:i])
			if err != nil {
				return 0, err
			}
			i -= size
			i = encodeVarintOperation(dAtA, i, uint64(size))
		}
		i--
		dAtA[i] = 0x22
	}
	return len(dAtA) - i, nil
}
func (m *InternalOperation) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *InternalOperation) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *InternalOperation) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if m.Operation != nil {
		{
			size := m.Operation.Size()
			i -= size
			if _, err := m.Operation.MarshalTo(dAtA[i:]); err != nil {
				return 0, err
			}
		}
	}
	return len(dAtA) - i, nil
}

func (m *InternalOperation_Match) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *InternalOperation_Match) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	if m.Match != nil {
		{
			size, err := m.Match.MarshalToSizedBuffer(dAtA[:i])
			if err != nil {
				return 0, err
			}
			i -= size
			i = encodeVarintOperation(dAtA, i, uint64(size))
		}
		i--
		dAtA[i] = 0xa
	}
	return len(dAtA) - i, nil
}
func (m *InternalOperation_ShortTermOrderPlacement) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *InternalOperation_ShortTermOrderPlacement) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	if m.ShortTermOrderPlacement != nil {
		{
			size, err := m.ShortTermOrderPlacement.MarshalToSizedBuffer(dAtA[:i])
			if err != nil {
				return 0, err
			}
			i -= size
			i = encodeVarintOperation(dAtA, i, uint64(size))
		}
		i--
		dAtA[i] = 0x12
	}
	return len(dAtA) - i, nil
}
func (m *InternalOperation_PreexistingStatefulOrder) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *InternalOperation_PreexistingStatefulOrder) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	if m.PreexistingStatefulOrder != nil {
		{
			size, err := m.PreexistingStatefulOrder.MarshalToSizedBuffer(dAtA[:i])
			if err != nil {
				return 0, err
			}
			i -= size
			i = encodeVarintOperation(dAtA, i, uint64(size))
		}
		i--
		dAtA[i] = 0x1a
	}
	return len(dAtA) - i, nil
}
func (m *InternalOperation_OrderRemoval) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *InternalOperation_OrderRemoval) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	if m.OrderRemoval != nil {
		{
			size, err := m.OrderRemoval.MarshalToSizedBuffer(dAtA[:i])
			if err != nil {
				return 0, err
			}
			i -= size
			i = encodeVarintOperation(dAtA, i, uint64(size))
		}
		i--
		dAtA[i] = 0x22
	}
	return len(dAtA) - i, nil
}
func encodeVarintOperation(dAtA []byte, offset int, v uint64) int {
	offset -= sovOperation(v)
	base := offset
	for v >= 1<<7 {
		dAtA[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	dAtA[offset] = uint8(v)
	return base
}
func (m *Operation) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.Operation != nil {
		n += m.Operation.Size()
	}
	return n
}

func (m *Operation_Match) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.Match != nil {
		l = m.Match.Size()
		n += 1 + l + sovOperation(uint64(l))
	}
	return n
}
func (m *Operation_ShortTermOrderPlacement) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.ShortTermOrderPlacement != nil {
		l = m.ShortTermOrderPlacement.Size()
		n += 1 + l + sovOperation(uint64(l))
	}
	return n
}
func (m *Operation_ShortTermOrderCancellation) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.ShortTermOrderCancellation != nil {
		l = m.ShortTermOrderCancellation.Size()
		n += 1 + l + sovOperation(uint64(l))
	}
	return n
}
func (m *Operation_PreexistingStatefulOrder) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.PreexistingStatefulOrder != nil {
		l = m.PreexistingStatefulOrder.Size()
		n += 1 + l + sovOperation(uint64(l))
	}
	return n
}
func (m *InternalOperation) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.Operation != nil {
		n += m.Operation.Size()
	}
	return n
}

func (m *InternalOperation_Match) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.Match != nil {
		l = m.Match.Size()
		n += 1 + l + sovOperation(uint64(l))
	}
	return n
}
func (m *InternalOperation_ShortTermOrderPlacement) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.ShortTermOrderPlacement != nil {
		l = m.ShortTermOrderPlacement.Size()
		n += 1 + l + sovOperation(uint64(l))
	}
	return n
}
func (m *InternalOperation_PreexistingStatefulOrder) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.PreexistingStatefulOrder != nil {
		l = m.PreexistingStatefulOrder.Size()
		n += 1 + l + sovOperation(uint64(l))
	}
	return n
}
func (m *InternalOperation_OrderRemoval) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.OrderRemoval != nil {
		l = m.OrderRemoval.Size()
		n += 1 + l + sovOperation(uint64(l))
	}
	return n
}

func sovOperation(x uint64) (n int) {
	return (math_bits.Len64(x|1) + 6) / 7
}
func sozOperation(x uint64) (n int) {
	return sovOperation(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *Operation) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowOperation
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= uint64(b&0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: Operation: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: Operation: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Match", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowOperation
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				msglen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthOperation
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthOperation
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			v := &ClobMatch{}
			if err := v.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			m.Operation = &Operation_Match{v}
			iNdEx = postIndex
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field ShortTermOrderPlacement", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowOperation
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				msglen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthOperation
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthOperation
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			v := &MsgPlaceOrder{}
			if err := v.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			m.Operation = &Operation_ShortTermOrderPlacement{v}
			iNdEx = postIndex
		case 3:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field ShortTermOrderCancellation", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowOperation
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				msglen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthOperation
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthOperation
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			v := &MsgCancelOrder{}
			if err := v.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			m.Operation = &Operation_ShortTermOrderCancellation{v}
			iNdEx = postIndex
		case 4:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field PreexistingStatefulOrder", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowOperation
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				msglen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthOperation
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthOperation
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			v := &OrderId{}
			if err := v.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			m.Operation = &Operation_PreexistingStatefulOrder{v}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipOperation(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthOperation
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func (m *InternalOperation) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowOperation
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= uint64(b&0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: InternalOperation: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: InternalOperation: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Match", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowOperation
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				msglen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthOperation
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthOperation
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			v := &ClobMatch{}
			if err := v.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			m.Operation = &InternalOperation_Match{v}
			iNdEx = postIndex
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field ShortTermOrderPlacement", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowOperation
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				msglen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthOperation
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthOperation
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			v := &MsgPlaceOrder{}
			if err := v.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			m.Operation = &InternalOperation_ShortTermOrderPlacement{v}
			iNdEx = postIndex
		case 3:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field PreexistingStatefulOrder", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowOperation
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				msglen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthOperation
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthOperation
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			v := &OrderId{}
			if err := v.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			m.Operation = &InternalOperation_PreexistingStatefulOrder{v}
			iNdEx = postIndex
		case 4:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field OrderRemoval", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowOperation
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				msglen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthOperation
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthOperation
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			v := &OrderRemoval{}
			if err := v.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			m.Operation = &InternalOperation_OrderRemoval{v}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipOperation(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthOperation
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func skipOperation(dAtA []byte) (n int, err error) {
	l := len(dAtA)
	iNdEx := 0
	depth := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowOperation
			}
			if iNdEx >= l {
				return 0, io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		wireType := int(wire & 0x7)
		switch wireType {
		case 0:
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return 0, ErrIntOverflowOperation
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				iNdEx++
				if dAtA[iNdEx-1] < 0x80 {
					break
				}
			}
		case 1:
			iNdEx += 8
		case 2:
			var length int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return 0, ErrIntOverflowOperation
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				length |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if length < 0 {
				return 0, ErrInvalidLengthOperation
			}
			iNdEx += length
		case 3:
			depth++
		case 4:
			if depth == 0 {
				return 0, ErrUnexpectedEndOfGroupOperation
			}
			depth--
		case 5:
			iNdEx += 4
		default:
			return 0, fmt.Errorf("proto: illegal wireType %d", wireType)
		}
		if iNdEx < 0 {
			return 0, ErrInvalidLengthOperation
		}
		if depth == 0 {
			return iNdEx, nil
		}
	}
	return 0, io.ErrUnexpectedEOF
}

var (
	ErrInvalidLengthOperation        = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowOperation          = fmt.Errorf("proto: integer overflow")
	ErrUnexpectedEndOfGroupOperation = fmt.Errorf("proto: unexpected end of group")
)