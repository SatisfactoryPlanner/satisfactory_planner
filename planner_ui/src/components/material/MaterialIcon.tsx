
type MaterialIconProps = {
  materialName: string;
  [x:string]: any;
}

export default function MaterialIcon(props: MaterialIconProps) {
  return (
    <img
      {...props}
      loading="lazy"
      src={`/item_icons/${props.materialName}.png`}
      alt={`${props.materialName} icon`}
    />
  )
}