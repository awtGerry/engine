use engine::algorithms::transformations::translate;

pub fn move_character(x: &mut f32, y: &mut f32, tx: f32, ty: f32) -> (f32, f32)
{
    (*x, *y) = translate(*x, *y, tx, ty);
    (*x, *y)
}
