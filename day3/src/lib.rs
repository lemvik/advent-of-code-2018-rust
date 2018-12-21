use common::errors::ParseError;
use common::parsers::*;
use std::iter::repeat;
use std::result::Result;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct ElfClaim {
    pub id: u32,
    pub position: (usize, usize),
    pub dimensions: (usize, usize),
}

impl ElfClaim {
    pub fn create(id: u32, position: (usize, usize), dimensions: (usize, usize)) -> ElfClaim {
        ElfClaim {
            id: id,
            position: position,
            dimensions: dimensions,
        }
    }

    pub fn parse(text: &str) -> Result<ElfClaim, ParseError> {
        let (id_text, rect_text) =
            split_pair(text, '@').ok_or(format!("Failed to split on @: {}", text))?;

        let claim_id = u32::from_str(id_text[1..].trim())?;
        let (pos, dims) =
            split_pair(rect_text, ':').ok_or(format!("Failed to split on ':': {}", rect_text))?;

        let position = parse_pair::<usize>(pos.trim(), ',')
            .ok_or(format!("Failed to parse coords: {}", pos))?;
        let dimensions = parse_pair::<usize>(dims.trim(), 'x')
            .ok_or(format!("Failed to parse dimensions: {}", dims))?;

        Ok(ElfClaim {
            id: claim_id,
            position: position,
            dimensions: dimensions,
        })
    }
}

pub fn parse_claims(source: Vec<String>) -> Result<Vec<ElfClaim>, ParseError> {
    source.into_iter().map(|s| ElfClaim::parse(&s)).collect()
}

pub fn boundaries(claims: &Vec<ElfClaim>) -> Option<(usize, usize)> {
    let xmax = claims
        .into_iter()
        .max_by_key(|cl| &cl.position.0 + &cl.dimensions.0)?;
    let ymax = claims
        .into_iter()
        .max_by_key(|cl| &cl.position.1 + &cl.dimensions.1)?;
    Some((
        xmax.position.0 + xmax.dimensions.0,
        ymax.position.1 + ymax.dimensions.1,
    ))
}

#[derive(Debug)]
pub struct DepthBuffer {
    pixels: Vec<Vec<u32>>,
}

impl DepthBuffer {
    pub fn create(dimensions: (usize, usize)) -> DepthBuffer {
        DepthBuffer {
            pixels: repeat(0)
                .take(dimensions.0)
                .map(|_| repeat(0).take(dimensions.1).collect())
                .collect(),
        }
    }

    pub fn add_claim(&mut self, claim: &ElfClaim) {
        let pos = claim.position;
        let dim = claim.dimensions;

        for i in pos.0..pos.0 + dim.0 {
            for j in pos.1..pos.1 + dim.1 {
                self.pixels[i][j] += 1;
            }
        }
    }

    pub fn count(&self, element: u32) -> usize {
        self.pixels
            .iter()
            .map(|c| c.iter().filter(|e| e > &&element).count())
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_claim_parsing() {
        assert_eq!(
            ElfClaim::parse("#1 @ 56,249: 24x16"),
            Ok(ElfClaim::create(1, (56, 249), (24, 16)))
        );
        assert!(match ElfClaim::parse("#1 56,249: 24x16") {
            Ok(_) => false,
            Err(_) => true,
        });
        assert!(match ElfClaim::parse("#1 @ 56r249: 24x16") {
            Ok(_) => false,
            Err(_) => true,
        });
        assert!(match ElfClaim::parse("#1 @ 56,249: 24y16") {
            Ok(_) => false,
            Err(_) => true,
        });
        assert!(match ElfClaim::parse("#1 @ 56,249 24y16") {
            Ok(_) => false,
            Err(_) => true,
        });
    }
}
