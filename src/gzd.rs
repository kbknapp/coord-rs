use ascii;
use col::ColLetter;
use row::RowLetter;
use band::LatBand;
use SET_ORIGIN_ROW_LETTERS;
use SET_ORIGIN_COLUMN_LETTERS;

/// Grid Zone Designator such as 28F comprised of a Zone Number (one or two digits) and Zone Letter
/// (C-X, minus I and O)
#[derive(Default, Copy, Clone, Debug)]
pub struct Gzd {
    /// 6° longitudinal zone (1..60 covering 180°W..180°E)
    pub zone: u8,
    /// 8° latitudinal band (C..X covering 80°S..84°N)
    pub band: LatBand,
}

/// 100k Grid Square ID such as FD
#[derive(Default, Copy, Clone, Debug)]
pub struct GridSquareId100k {
    pub col: ColLetter,
    pub row: RowLetter
}

impl GridSquareId100k {
    pub fn new(column: u32, row: u32, parm: usize) -> GridSquareId100k {
        /*!
        Get the two-letter MGRS 100k designator given information translated from the UTM northing,
        easting and zone number.

        ### Params
         * **column**: the column index as it relates to the MGRS 100k set spreadsheet, created from
                       the UTM easting. Values are 1-8.
         * **row**: the row index as it relates to the MGRS 100k set spreadsheet, created from the UTM
                    northing value. Values are from 0-19.
         * **parm**: the set block, as it relates to the MGRS 100k set spreadsheet, created from the UTM
                    zone. Values are from 1-60.
        ### Return
         * two letter MGRS 100k code as a `GridSquareId100k`.
        */

        // col_origin and row_origin are the letters at the origin of the set
        let index = parm - 1;
        let col_origin = SET_ORIGIN_COLUMN_LETTERS[index];
        let row_origin = SET_ORIGIN_ROW_LETTERS[index];

        // col_int and row_int are the letters to build to return
        let mut col_int = col_origin + column as u8 - 1;
        let mut row_int = row_origin + row as u8;
        let mut rollover = false;

        if col_int > ascii::Z {
            col_int = col_int - ascii::Z + ascii::A - 1;
            rollover = true;
        }

        if col_int == ascii::I || (col_origin < ascii::I && col_int > ascii::I) || ((col_int > ascii::I || col_origin < ascii::I) && rollover) {
            col_int += 1;
        }

        if col_int == ascii::O || (col_origin < ascii::O && col_int > ascii::O) || ((col_int > ascii::O || col_origin < ascii::O) && rollover) {
            col_int += 1;

            if col_int == ascii::I {
                col_int += 1;
            }
        }

        if col_int > ascii::Z {
            col_int = col_int - ascii::Z + ascii::A - 1;
        }

        if row_int > ascii::V {
            row_int = row_int - ascii::V + ascii::A - 1;
            rollover = true;
        } else {
            rollover = false;
        }

        if ((row_int == ascii::I) || ((row_origin < ascii::I) && (row_int > ascii::I))) || (((row_int > ascii::I) || (row_origin < ascii::I)) && rollover) {
            row_int += 1;
        }

        if ((row_int == ascii::O) || ((row_origin < ascii::O) && (row_int > ascii::O))) || (((row_int > ascii::O) || (row_origin < ascii::O)) && rollover) {
            row_int += 1;

            if row_int == ascii::I {
                row_int += 1;
            }
        }

        if row_int > ascii::V {
            row_int = row_int - ascii::V + ascii::A - 1;
        }

        GridSquareId100k {
            col: ColLetter::from(col_int as u32),
            row: RowLetter::from(row_int as u32)
        }
    }
}
