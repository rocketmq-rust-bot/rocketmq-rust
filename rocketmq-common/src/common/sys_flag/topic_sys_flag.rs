/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

const FLAG_UNIT: u32 = 0x1 << 0;
const FLAG_UNIT_SUB: u32 = 0x1 << 1;

pub fn build_sys_flag(unit: bool, has_unit_sub: bool) -> u32 {
    let mut sys_flag = 0;
    if unit {
        sys_flag |= FLAG_UNIT;
    }
    if has_unit_sub {
        sys_flag |= FLAG_UNIT_SUB;
    }
    sys_flag
}

pub fn set_unit_flag(sys_flag: u32) -> u32 {
    sys_flag | FLAG_UNIT
}

pub fn clear_unit_flag(sys_flag: u32) -> u32 {
    sys_flag & !FLAG_UNIT
}

pub fn has_unit_flag(sys_flag: u32) -> bool {
    (sys_flag & FLAG_UNIT) == FLAG_UNIT
}

pub fn set_unit_sub_flag(sys_flag: u32) -> u32 {
    sys_flag | FLAG_UNIT_SUB
}

pub fn clear_unit_sub_flag(sys_flag: u32) -> u32 {
    sys_flag & !FLAG_UNIT_SUB
}

pub fn has_unit_sub_flag(sys_flag: u32) -> bool {
    (sys_flag & FLAG_UNIT_SUB) == FLAG_UNIT_SUB
}
