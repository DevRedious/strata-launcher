// Icône de chaque Discord, lue dans son installation (app.ico) : la vraie icône de
// Stable, PTB ou Canary, sans embarquer de logo de Discord dans le launcher.
//
// app.ico pèse près de 300 Ko, surtout à cause de son image de 256 px. On ne garde
// que les images de 64 px au plus, assez pour une carte de 44 px, et l'interface
// l'affiche telle quelle (WebView2 lit le format ICO).

use std::fs;
use std::path::Path;

const MAX_SIZE: u32 = 64;

/// Nouvelle icône avec les seules images d'au plus `MAX_SIZE` px, ou None si le fichier
/// n'est pas un ICO valide. Format : en-tête de 6 octets, une entrée de 16 octets par
/// image (largeur, hauteur, …, taille, position), puis les images.
fn small_ico(ico: &[u8]) -> Option<Vec<u8>> {
    let u16_at = |at: usize| Some(u16::from_le_bytes(ico.get(at..at + 2)?.try_into().ok()?));
    let u32_at = |at: usize| Some(u32::from_le_bytes(ico.get(at..at + 4)?.try_into().ok()?));
    if u16_at(0)? != 0 || u16_at(2)? != 1 {
        return None;
    }

    let mut kept = Vec::new();
    for index in 0..usize::from(u16_at(4)?) {
        let entry = 6 + index * 16;
        let width = match *ico.get(entry)? {
            0 => 256,
            w => u32::from(w),
        };
        let (size, offset) = (u32_at(entry + 8)? as usize, u32_at(entry + 12)? as usize);
        if width <= MAX_SIZE {
            kept.push((ico.get(entry..entry + 8)?, ico.get(offset..offset + size)?));
        }
    }
    if kept.is_empty() {
        return None;
    }

    let mut out = vec![0, 0, 1, 0];
    out.extend_from_slice(&(kept.len() as u16).to_le_bytes());
    let mut offset = 6 + kept.len() * 16;
    for (head, image) in &kept {
        out.extend_from_slice(head);
        out.extend_from_slice(&(image.len() as u32).to_le_bytes());
        out.extend_from_slice(&(offset as u32).to_le_bytes());
        offset += image.len();
    }
    for (_, image) in &kept {
        out.extend_from_slice(image);
    }
    Some(out)
}

fn base64(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let n = chunk
            .iter()
            .enumerate()
            .fold(0u32, |n, (i, &b)| n | u32::from(b) << (16 - 8 * i));
        for i in 0..4 {
            out.push(if i <= chunk.len() {
                TABLE[(n >> (18 - 6 * i) & 63) as usize] as char
            } else {
                '='
            });
        }
    }
    out
}

/// Icône prête pour une balise <img> (data URL), si le Discord en a une
pub fn discord_icon(root: &Path) -> Option<String> {
    let ico = small_ico(&fs::read(root.join("app.ico")).ok()?)?;
    Some(format!("data:image/x-icon;base64,{}", base64(&ico)))
}

#[cfg(test)]
mod tests {
    use super::*;

    // ICO factice : une image par taille demandée, remplie de son numéro
    fn fake_ico(widths: &[u8]) -> Vec<u8> {
        let mut out = vec![0, 0, 1, 0, widths.len() as u8, 0];
        let mut offset = 6 + widths.len() * 16;
        for (i, &w) in widths.iter().enumerate() {
            out.extend_from_slice(&[w, w, 0, 0, 1, 0, 32, 0]);
            out.extend_from_slice(&((i + 1) as u32 * 10).to_le_bytes());
            out.extend_from_slice(&(offset as u32).to_le_bytes());
            offset += (i + 1) * 10;
        }
        for i in 0..widths.len() {
            out.extend(std::iter::repeat_n(i as u8, (i + 1) * 10));
        }
        out
    }

    #[test]
    fn keeps_only_the_small_images() {
        // 16, 32, 48 et 256 px (codé 0), comme l'app.ico de Discord
        let small = small_ico(&fake_ico(&[16, 32, 48, 0])).unwrap();
        assert_eq!(small, fake_ico(&[16, 32, 48]));
    }

    #[test]
    fn rejects_what_is_not_an_icon() {
        assert_eq!(small_ico(b"\x89PNG not an icon"), None);
        assert_eq!(small_ico(&fake_ico(&[0])), None);
    }

    #[test]
    fn encodes_base64() {
        assert_eq!(base64(b""), "");
        assert_eq!(base64(b"f"), "Zg==");
        assert_eq!(base64(b"fo"), "Zm8=");
        assert_eq!(base64(b"foo"), "Zm9v");
        assert_eq!(base64(b"Strata!"), "U3RyYXRhIQ==");
    }
}
