using System;
using System.Collections.Generic;
using System.Linq;
using OpenTK.Mathematics;

readonly struct TileAndPosition
{
    public Tile Tile { get; }
    public Position Position { get; }

    public TileAndPosition(Tile tile, Position position)
    {
        Tile = tile;
        Position = position;
    }
}

public class Map
{
    private readonly List<TileAndPosition> _tiles = new();
    private readonly Vector3i _noiseOffsets;

    public Map()
    {
        var random = new Random();

        _noiseOffsets = new Vector3i(random.Next(int.MinValue, int.MaxValue), random.Next(int.MinValue, int.MaxValue),
            random.Next(int.MinValue, int.MaxValue));
    }

    // Returns a two-dimensional array of tiles, with the y being the first layer and the x being the second.
    public List<List<Tile>> GetNearbyTiles(Position position, int radiusX, int radiusY)
    {
        var nearbyTiles = new List<List<Tile>>();

        for (int y = position.Y + radiusY; y >= position.Y - radiusY; y--)
        {
            nearbyTiles.Add(new List<Tile>());
            for (int x = position.X - radiusX; x <= position.X + radiusX; x++)
            {
                nearbyTiles.Last().Add(GetTile(new Position(x, y)));
            }
        }

        return nearbyTiles;
    }

    private Tile GetTile(Position position)
    {
        Tile foundTile;

        // This is absolutely terrible efficiency, but it's fine for the assignment.
        int tileIndex = _tiles.FindIndex((tile) => tile.Position.X == position.X && tile.Position.Y == position.Y);
        if (tileIndex != -1)
        {
            foundTile = _tiles[tileIndex].Tile;
        }
        else
        {
            // Generate the tile if it doesn't already exist.
            foundTile = GenerateTile(position.X, position.Y);
            _tiles.Add(new TileAndPosition(foundTile, new Position(position.X, position.Y)));
        }

        return foundTile;
    }

    public bool PositionIsWalkable(Position position)
    {
        return GetTile(position).IsWalkable();
    }

    private Tile GenerateTile(int x, int y)
    {
        var noise = PerlinNoise.OctavedNoise(x + _noiseOffsets.X, y + _noiseOffsets.Y, _noiseOffsets.Z, scale: 30);

        var height = noise + 0.05;

        // Make the starting area always land
        if (Math.Sqrt(x * x + y * y) < 10)
        {
            return new GroundTile();
        }

        return height switch
        {
            < 0 => new WaterTile(),
            < 0.04 => new BeachTile(),
            < 0.3 => new GroundTile(),
            _ => new MountainTile()
        };
    }
}
