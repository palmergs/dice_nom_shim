# frozen_string_literal: true

require 'json'

RSpec.describe DiceNomShim do
  it "has a version number" do
    expect(DiceNomShim::VERSION).not_to be nil
  end

  it "rolls dice" do
    hsh = JSON.parse(DiceNomShim.roll("2d6"))
    puts hsh
    expect(hsh[0].dig("lhs", "total")).to be >= 2
  end

  it "rolls percentile dice" do
    hsh = JSON.parse(DiceNomShim.roll("d100"))
    puts hsh
    expect(hsh[0].dig("lhs", "total")).to be >= 1
  end

  it "rolls arbitrary dice" do
    hsh = JSON.parse(DiceNomShim.roll("10d17"))
    puts hsh[0].dig("lhs", "values")
    expect(hsh[0].dig("lhs", "total")).to be >= 5
  end

  it "builds a histogram" do
    hsh = JSON.parse(DiceNomShim.histo("2d6**"))
    puts hsh
    expect(hsh[0]["value"]).to eq(2)
  end
end
