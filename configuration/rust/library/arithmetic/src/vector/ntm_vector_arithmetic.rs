///////////////////////////////////////////////////////////////////////////////////
//                                            __ _      _     _                  //
//                                           / _(_)    | |   | |                 //
//                __ _ _   _  ___  ___ _ __ | |_ _  ___| | __| |                 //
//               / _` | | | |/ _ \/ _ \ '_ \|  _| |/ _ \ |/ _` |                 //
//              | (_| | |_| |  __/  __/ | | | | | |  __/ | (_| |                 //
//               \__, |\__,_|\___|\___|_| |_|_| |_|\___|_|\__,_|                 //
//                  | |                                                          //
//                  |_|                                                          //
//                                                                               //
//                                                                               //
//              Peripheral-NTM for MPSoC                                         //
//              Neural Turing Machine for MPSoC                                  //
//                                                                               //
///////////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////////
//                                                                               //
// Copyright (c) 2020-2024 by the author(s)                                      //
//                                                                               //
// Permission is hereby granted, free of charge, to any person obtaining a copy  //
// of this software and associated documentation files (the "Software"), to deal //
// in the Software without restriction, including without limitation the rights  //
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell     //
// copies of the Software, and to permit persons to whom the Software is         //
// furnished to do so, subject to the following conditions:                      //
//                                                                               //
// The above copyright notice and this permission notice shall be included in    //
// all copies or substantial portions of the Software.                           //
//                                                                               //
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR    //
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,      //
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE   //
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER        //
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, //
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN     //
// THE SOFTWARE.                                                                 //
//                                                                               //
// ============================================================================= //
// Author(s):                                                                    //
//   Paco Reina Campo <pacoreinacampo@queenfield.tech>                           //
//                                                                               //
///////////////////////////////////////////////////////////////////////////////////

pub struct VectorArithmetic {
    pub data_a_in: Vec<f64>,
    pub data_b_in: Vec<f64>,

    pub data_out: Vec<f64>
}

impl VectorArithmetic {
    pub fn ntm_vector_adder(&self) -> Vec<f64> {
        // Add two vectors of identical dimensions
        let mut data_out: Vec<f64> = vec![];

        if self.data_a_in.len() != self.data_b_in.len() {
            panic!("Vector dimensions do not match");
        }

        for i in 0..self.data_a_in.len() {
            let temporal = self.data_a_in[i] + self.data_b_in[i];

            data_out.push(temporal);
        }
        data_out
    }

    pub fn ntm_vector_subtractor(&self) -> Vec<f64> {
        // Add two vectors of identical dimensions
        let mut data_out: Vec<f64> = vec![];

        if self.data_a_in.len() != self.data_b_in.len() {
            panic!("Vector dimensions do not match");
        }

        for i in 0..self.data_a_in.len() {
            let temporal = self.data_a_in[i] - self.data_b_in[i];

            data_out.push(temporal);
        }
        data_out
    }

    pub fn ntm_vector_multiplier(&self) -> Vec<f64> {
        // Multiply two vectors of identical dimensions
        let mut data_out: Vec<f64> = vec![];

        if self.data_a_in.len() != self.data_b_in.len() {
            panic!("Vector dimensions do not match");
        }

        for i in 0..self.data_a_in.len() {
            let temporal = self.data_a_in[i] * self.data_b_in[i];

            data_out.push(temporal);
        }
        data_out
    }

    pub fn ntm_vector_divider(&self) -> Vec<f64> {
        // Divide two vectors of identical dimensions
        let mut data_out: Vec<f64> = vec![];

        if self.data_a_in.len() != self.data_b_in.len() {
            panic!("Vector dimensions do not match");
        }

        for i in 0..self.data_a_in.len() {
            let temporal = self.data_a_in[i] / self.data_b_in[i];

            data_out.push(temporal);
        }
        data_out
    }
}
