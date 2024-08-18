# coding: utf-8

"""
    Binance Public Spot API

    OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)

    The version of the OpenAPI document: 1.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


from __future__ import annotations
import pprint
import re  # noqa: F401
import json

from pydantic import BaseModel, ConfigDict, Field, StrictStr
from typing import Any, ClassVar, Dict, List
from openapi_client.models.snapshot_spot_snapshot_vos_inner_data_balances_inner import SnapshotSpotSnapshotVosInnerDataBalancesInner
from typing import Optional, Set
from typing_extensions import Self

class SnapshotSpotSnapshotVosInnerData(BaseModel):
    """
    SnapshotSpotSnapshotVosInnerData
    """ # noqa: E501
    balances: List[SnapshotSpotSnapshotVosInnerDataBalancesInner]
    total_asset_of_btc: StrictStr = Field(alias="totalAssetOfBtc")
    __properties: ClassVar[List[str]] = ["balances", "totalAssetOfBtc"]

    model_config = ConfigDict(
        populate_by_name=True,
        validate_assignment=True,
        protected_namespaces=(),
    )


    def to_str(self) -> str:
        """Returns the string representation of the model using alias"""
        return pprint.pformat(self.model_dump(by_alias=True))

    def to_json(self) -> str:
        """Returns the JSON representation of the model using alias"""
        # TODO: pydantic v2: use .model_dump_json(by_alias=True, exclude_unset=True) instead
        return json.dumps(self.to_dict())

    @classmethod
    def from_json(cls, json_str: str) -> Optional[Self]:
        """Create an instance of SnapshotSpotSnapshotVosInnerData from a JSON string"""
        return cls.from_dict(json.loads(json_str))

    def to_dict(self) -> Dict[str, Any]:
        """Return the dictionary representation of the model using alias.

        This has the following differences from calling pydantic's
        `self.model_dump(by_alias=True)`:

        * `None` is only added to the output dict for nullable fields that
          were set at model initialization. Other fields with value `None`
          are ignored.
        """
        excluded_fields: Set[str] = set([
        ])

        _dict = self.model_dump(
            by_alias=True,
            exclude=excluded_fields,
            exclude_none=True,
        )
        # override the default output from pydantic by calling `to_dict()` of each item in balances (list)
        _items = []
        if self.balances:
            for _item in self.balances:
                if _item:
                    _items.append(_item.to_dict())
            _dict['balances'] = _items
        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of SnapshotSpotSnapshotVosInnerData from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "balances": [SnapshotSpotSnapshotVosInnerDataBalancesInner.from_dict(_item) for _item in obj["balances"]] if obj.get("balances") is not None else None,
            "totalAssetOfBtc": obj.get("totalAssetOfBtc")
        })
        return _obj


