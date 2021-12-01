package lib

import (
	"io"
	"reflect"
	"strings"
	"testing"
)

func TestSumInts(t *testing.T) {
	type args struct {
		in []int
	}
	tests := []struct {
		name    string
		args    args
		wantOut int
	}{
		{
			name: "Zero",
			args: args{
				in: []int{},
			},
			wantOut: 0,
		},
		{
			name: "Simple",
			args: args{
				in: []int{1, 2, 3},
			},
			wantOut: 6,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if gotOut := SumInts(tt.args.in); gotOut != tt.wantOut {
				t.Errorf("SumInts() = %v, want %v", gotOut, tt.wantOut)
			}
		})
	}
}

func TestReadInts(t *testing.T) {
	type args struct {
		r io.Reader
	}
	tests := []struct {
		name    string
		args    args
		want    []int
		wantErr bool
	}{
		{
			name: "Empty",
			args: args{
				r: strings.NewReader(""),
			},
			want:    []int{},
			wantErr: false,
		},
		{
			name: "NewLine",
			args: args{
				r: strings.NewReader("1\n2\n3\n"),
			},
			want:    []int{1, 2, 3},
			wantErr: false,
		},
		{
			name: "Spaces",
			args: args{
				r: strings.NewReader("1 2 3"),
			},
			want:    []int{1, 2, 3},
			wantErr: false,
		},
		{
			name: "Tabs",
			args: args{
				r: strings.NewReader("1\t2\t3"),
			},
			want:    []int{1, 2, 3},
			wantErr: false,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := ReadInts(tt.args.r)
			if (err != nil) != tt.wantErr {
				t.Errorf("ReadInts() error = %v, wantErr %v", err, tt.wantErr)
				return
			}
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("ReadInts() = %v, want %v", got, tt.want)
			}
		})
	}
}
